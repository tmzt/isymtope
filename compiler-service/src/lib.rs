extern crate futures;
extern crate tokio_core;

extern crate isymtope_ast_common;
extern crate isymtope_build;
extern crate isymtope_generate;

pub mod context;
pub mod messages;
pub mod compile;

pub use self::context::*;
pub use self::messages::*;

use std::thread;

use tokio_core::reactor::Core;
use futures::future;

use isymtope_generate::*;
use self::compile::*;

pub type CompilerRequestChannel = futures::sync::mpsc::UnboundedSender<(CompilerRequestMsg, CompilerResponseChannel)>;
pub type CompilerResponseChannel = futures::sync::oneshot::Sender<IsymtopeGenerateResult<CompilerResponseMsg>>;

pub fn spawn_compiler_service() -> IsymtopeGenerateResult<CompilerRequestChannel> {
    let (sender, receiver) = futures::sync::mpsc::unbounded();

    thread::spawn(move || {
        use futures::Stream;
        let mut core = Core::new().unwrap();
        let mut shared_ctx = DefaultCompilerContext::new();

        core.run(
            receiver.for_each(move |(msg, oneshot): (_, CompilerResponseChannel)| {
                let response = shared_ctx.handle_msg(msg);
                oneshot.send(response).ok().unwrap();
                // if r.is_err() {
                //     return future::err(IOError::new(
                //         IOErrorKind::Other,
                //         "Failed making request on ServerContext.",
                //     ));
                // };

                future::ok(())
            }),
        ).ok()
            .unwrap();
    });

    Ok(sender)
}
