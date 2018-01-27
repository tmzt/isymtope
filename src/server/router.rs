
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};
use std::path::Path;
use std::fmt::Debug;

use hyper::{Response, StatusCode};
use regex::RegexSet;

use common::*;
use expressions::*;
use server::*;


#[derive(Debug, Default)]
pub struct Router {
    routes: Option<RegexSet>
}

impl Router {
    pub fn with_document_provider(document_provider: Rc<DocumentProvider>) -> Self {
        let doc = document_provider.doc();
        let routes = doc.routes();

        let route_patterns: Vec<_> = routes.map(|r| r.pattern()).collect();
        let routes = RegexSet::new(route_patterns.into_iter()).unwrap();

        Router { routes: Some(routes) }
    }

    // // fn prepare_session<'s>(&self, ctx: &'s mut ServerContext, doc: &Document, path: Option<&str>) -> IsymtopeServerResult<ReturnedSession<'s>> {
    // fn prepare_session(&self, ctx: &mut ServerContext, doc: &Document, path: Option<&str>) -> IsymtopeServerResult<()> {
    //     // Random cookie
    //     // let cookie = ctx.create_key(SESSION_COOKIES_RANDOM_STRING_BYTES)?;

    //     let msg = Msg::NewSession(SESSION_COOKIES_RANDOM_STRING_BYTES);
    //     ctx.handle_msg(msg)?;

    //     // Get session for the generated cookie
    //     // let mut returned_session = ctx.get_session(&cookie)?;

    //     // Initialize state and dispatch initial actions
    //     // let exec = &self.executor;
    //     let reducers = doc.reducers();

    //     // exec.initialize_reducer_state_with_default_exprs(&mut session, reducers);

    //     // Fake routing
    //     let route_tag = match path { Some("/completed") => "completed", Some("/active") => "active", _ => "all" };
    //     eprintln!("Setting SHOWING.FILTER to {}", &route_tag);

    //     // let route_tag = ExpressionValue::Primitive(Primitive::StringVal(route_tag.to_owned()));

    //     // returned_session.session().set_value("showing", Some(route_tag), true)?;
    //     let msg = Msg::SetValueInSession("showing".to_owned(), Some(route_tag.to_owned()), true);
    //     ctx.handle_msg(msg)?;

    //     // Ok(returned_session)
    //     Ok(())
    // }

    // pub fn handle_request(&self, ctx: &mut ServerContext, w: &mut io::Write, doc: &Document, req: <<Self as ServiceInject>::ServiceImpl as Service>::Request, path: Option<&str>) -> IsymtopeServerResult<()> {
    //     // let session: Rc<_> = Rc::new(self.prepare_session(ctx, doc, path)?);
    //     self.prepare_session(ctx, doc, path)?;

    //     // Reducer state
    //     // let provider: Rc<ReducerStateProvider> = Rc::new(SessionReducerStateProvider::new(&session));
    //     // let mut ctx = Context::with_state_provider(&provider);

    //     // Render page
    //     let mut html_writer = DefaultHtmlWriter::default();
    //     let mut out_ctx: OutputContext<ProcessedExpression> = Default::default();

    //     html_writer.write_object(w, &mut out_ctx, doc)?;

    //     Ok(())
    // }

    // fn handle_route(&mut self, doc: &Document, req: <<Self as RouterTrait>::ServiceImpl as Service>::Request, path: Option<&str>) -> <<Self as RouterTrait>::ServiceImpl as Service>::Future {
    //     let mut buf: Vec<u8> = Default::default();
    //     let res = self.do_route(&mut buf, doc, req, path);

    //     match res {
    //         Err(e) => {
    //             futures::future::ok(
    //                 Response::new()
    //                     .with_status(StatusCode::InternalServerError)
    //                     .with_body(format!("Error occurred processing page: {:?}", e)))
    //         }

    //         _ => {
    //             futures::future::ok(
    //                 Response::new()
    //                     .with_body(buf))
    //         }
    //     }
    // }

    // pub fn handle_call(&mut self, doc: &Document, req: <<Self as RouterTrait>::ServiceImpl as Service>::Request) -> <<Self as RouterTrait>::ServiceImpl as Service>::Future {
 
    //     let path = req.path().to_owned();
    //     println!("Path requested: {}", &path);

    //     let has_match = self.routes.as_ref().map(|r| r.matches(&path).matched_any()).unwrap_or_default();
    //     if has_match {
    //         println!("Matched route: {}", &path);
    //         return self.handle_route(doc, req, Some(&path));
    //     };

    //     println!("Serving default route for {}", &path);
    //     self.handle_route(doc, req, None)
    // }
}