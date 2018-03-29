use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::str;

use futures::{self, future, Future, Stream};
use hyper::{Request, Response};
use hyper::Method::Post;
use hyper::header::ContentType;
use hyper::mime;
use hyper::Error as HyperError;
use hyper::StatusCode;
use hyper_staticfile::Static;
use regex::RegexSet;

use tokio_core::reactor::Handle;

use isymtope_build::*;
use isymtope_generate::*;
use compiler_service::{*, CompilerContext};
use super::*;

#[derive(Debug)]
pub struct PlaygroundApiServiceFactory {
    sender: CompilerRequestChannel,
    handle: Handle,
}

impl PlaygroundApiServiceFactory {
    pub fn new(sender: CompilerRequestChannel, handle: Handle) -> Self {
        PlaygroundApiServiceFactory {
            sender: sender,
            handle: handle,
        }
    }
}

impl IsymtopeAppServiceFactory for PlaygroundApiServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Instance = PlaygroundApiService;

    fn create(&self) -> Self::Instance {
        PlaygroundApiService {
            sender: self.sender.clone(),
            handle: self.handle.clone(),
        }
    }
}

#[derive(Debug)]
pub struct PlaygroundApiService {
    sender: CompilerRequestChannel,
    handle: Handle,
}

// fn process_body(req: Request) -> impl Future<Item = String, Error = HyperError> {
//     req.body()
//         .map_err(|_|
//             HyperError::Io(IOError::new(
//                 IOErrorKind::Other,
//                 "Failed making request to compiler service.",
//             ))
//         )
//         .map(|chunk| {
//             println!("==> CHUNK: {}", String::from_utf8(chunk.to_vec()).unwrap());
//             chunk
//         })
//         // .fold(Vec::with_capacity(8192 * 4), |mut vector, chunk| {
//         .fold(Vec::new(), |mut vector, chunk| {
//             vector.extend_from_slice(&chunk);

//             Ok(vector)
//         })
//         .and_then(|vector| future::result(String::from_utf8(vector))
//             .map_err(|_: std::string::FromUtf8Error|
//                 HyperError::Io(IOError::new(
//                     IOErrorKind::Other,
//                     "Failed making request to compiler service.",
//                 ))))

//     // req.body().concat2().and_then(|s| str::from_utf8(&s)
//     //                             .map_err(|_| {
//     //                                     HyperError::Io(IOError::new(
//     //                                         IOErrorKind::Other,
//     //                                         "Failed making request to compiler service.",
//     //                                     ))
//     //                             })
//     //                             .map(|s| s.to_owned()))
// }

fn send_request(sender: &CompilerRequestChannel, source: &str, app_name: &str) -> impl Future<Item = IsymtopeGenerateResult<CompilerResponseMsg>, Error = HyperError> {
    // let app_base_url = "/app/playground/preview-1bcx1".into();
    // let app_name: String = "todomvc".into();
    let app_base_url = format!("/app/playground/_worker/app/{}", app_name);
    let compiler_req = CompilerRequestMsg::CompileSource(source.to_owned(), app_base_url);

    let (tx, rx) = futures::sync::oneshot::channel::<IsymtopeGenerateResult<CompilerResponseMsg>>();
    future::result(sender.unbounded_send((compiler_req, tx)))
        .map_err(|_|
            HyperError::Io(IOError::new(
                IOErrorKind::Other,
                "Failed making request to compiler service.",
            )))
        .and_then(move |_| rx.map_err(|_|
            HyperError::Io(IOError::new(
                IOErrorKind::Other,
                "Failed making request to compiler service.",
            ))))
}

fn make_response(compiler_resp: IsymtopeGenerateResult<CompilerResponseMsg>) -> impl Future<Item = Response, Error = HyperError> {
    let response;
    println!("[playground api] compiler_resp: {:?}", compiler_resp);

    match compiler_resp {
        Ok(CompilerResponseMsg::CompileComplete(body)) => {
            response = Response::new()
                .with_body(body)
                .with_header(ContentType(mime::TEXT_HTML));
        }

        _ => {
            let error_msg = "Error compiling template";
            response = Response::new()
                .with_body(error_msg)
                .with_header(ContentType(mime::TEXT_HTML))
                .with_status(StatusCode::InternalServerError);
        }
    };

    future::ok(response)
}

impl IsymtopeAppService for PlaygroundApiService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, base_url: &str, app_name: &str, req: Request) -> Self::Future {
        match req.method() {
            &Post => {
                let sender = self.sender.clone();
                let app_name = app_name.to_owned();

                let response = req.body()
                    .concat2()
                    .and_then(move |chunk| String::from_utf8(chunk.to_vec())
                        .map_err(|_|
                            HyperError::Io(IOError::new(
                                IOErrorKind::Other,
                                "Failed making request to compiler service.",
                            )))
                    )
                    .and_then(move |source| self::send_request(&sender, &source, &app_name))
                    .and_then(move |compile_resp| self::make_response(compile_resp));

                Box::new(response)
            }

            _ => {
                Box::new(future::ok(Response::new().with_status(StatusCode::MethodNotAllowed)))
            }
        }
    }
}
