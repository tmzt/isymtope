
use std::env;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read};
use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::sync::Mutex;
use std::error::Error;

use time::Duration;
use futures::{self, Future};
use hyper::Error as HyperError;
use regex::RegexSet;

use isymtope_build::input::*;
use isymtope_build::processing::*;
use server::*;


pub type IsymtopeServiceRouter = Router;
pub type ResponseMsgChannel = futures::sync::oneshot::Sender<IsymtopeServerResult<ResponseMsg>>;
pub type RequestMsgChannel = futures::sync::mpsc::UnboundedSender<(Msg, ResponseMsgChannel)>;

#[derive(Debug)]
pub struct IsymtopeServiceFactory {
    sender: RequestMsgChannel
}

impl IsymtopeServiceFactory {
    pub fn new(sender: RequestMsgChannel) -> Self {
        IsymtopeServiceFactory { sender: sender }
    }
}

impl NewService for IsymtopeServiceFactory {
    type Request = <Self::Instance as Service>::Request;
    type Response = <Self::Instance as Service>::Response;
    type Error = <Self::Instance as Service>::Error;
    type Instance = IsymtopeService;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(IsymtopeService { sender: self.sender.clone() })
    }
}

#[derive(Debug)]
pub struct IsymtopeService {
    sender: RequestMsgChannel
}

impl Service for IsymtopeService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut buf = String::with_capacity(4096);
        if let Some(resource_dir) = env::var_os("RESOURCE_DIR") {
            let resource_dir = Path::new(&resource_dir);
            let trimmed_path = req.path().trim_left_matches('/');
            let resource_path = resource_dir.join(&trimmed_path);

            if resource_path.is_file() {
                eprintln!("[service] Serving resource path: {:?}", resource_path);
                File::open(resource_path).and_then(|mut f| f.read_to_string(&mut buf)).ok();

                let response = Response::new().with_body(buf);
                return Box::new(future::ok(response));
            };
        };

        let session_expires = Some(Duration::days(1));

        let (tx1, rx1) = futures::sync::oneshot::channel::<IsymtopeServerResult<ResponseMsg>>();
        let new_session = Msg::NewSession(SESSION_COOKIES_RANDOM_STRING_BYTES, session_expires);

        let (tx2, rx2) = futures::sync::oneshot::channel::<IsymtopeServerResult<ResponseMsg>>();
        let execute_route = Msg::ExecuteRoute(format!("{}", req.path()));

        let (tx3, rx3) = futures::sync::oneshot::channel::<IsymtopeServerResult<ResponseMsg>>();
        let render = Msg::RenderRoute(format!("{}", req.path()));

        self.sender.unbounded_send((new_session, tx1)).unwrap();
        self.sender.unbounded_send((execute_route, tx2)).unwrap();
        self.sender.unbounded_send((render, tx3)).unwrap();

        let work = rx1.join3(rx2, rx3)
            .map_err(|_| HyperError::Io(IOError::new(IOErrorKind::Other, "Failed making request on ServerContext.")))
            .and_then(move |(session_response, routed, rendered)| {
                eprintln!("Got session created or validated: {:?}", session_response);
                eprintln!("Got route complete: {:?}", routed);
                eprintln!("Got render complete.");

                eprintln!("Got render result: {:?}", rendered);

                // let body = match rendered { Ok(ResponseMsg::RenderComplete(response)) => Some(response), _ => None }.unwrap().take();
                // let response = Response::new().with_body(body);
                // future::ok(response)

                match rendered {
                    Ok(ResponseMsg::RenderComplete(response)) => {
                        let body = response.take();
                        let response = Response::new().with_body(body);
                        future::ok(response)
                    }

                    _ => {
                        let body = format!("Unknown response message from render task: {:?}", rendered);
                        let response = Response::new().with_body(body);
                        future::ok(response)
                    }
                }
            });

        Box::new(work)
    }
}