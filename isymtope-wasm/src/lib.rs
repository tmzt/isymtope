#![feature(wasm_import_memory, panic_handler, std_panic)]
#![wasm_import_memory]

#[macro_use]
extern crate log;
#[macro_use(eprint, eprintln)]
extern crate wasm_log;
#[macro_use]
extern crate lazy_static;

extern crate isymtope_ast_common;
extern crate isymtope_build;

use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;
use std::sync::Mutex;

use isymtope_ast_common::*;
use isymtope_build::*;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

lazy_static!(
    // static ref FACTORY: Mutex<InternalTemplateRendererFactory> = Mutex::new(InternalTemplateRendererFactory::create().expect("error creating internal renderer factory"));
    static ref FACTORY: Mutex<InternalTemplateRendererFactory> = Default::default();
);

#[no_mangle]
pub extern "C" fn compile_template(data: *mut c_char) -> *mut c_char {
    // wasm_log::wasm_log_init();
    // wasm_log::wasm_install_panic_hook();
    // panic::set_hook(Box::new(|panic_info| eprintln!("panic occurred: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap())));

    let cstr = unsafe { CString::from_raw(data) };
    let src = cstr.to_string_lossy();
    let template = Rc::new(parser::parse_str(&src).unwrap());
    eprintln!("Parsed document");
    let mut ctx: DefaultProcessingContext<ProcessedExpression> =
        DefaultProcessingContext::for_template(template.clone());
    eprintln!("Created processing context");
    let document: Document = TryProcessFrom::try_process_from(template.as_ref(), &mut ctx).unwrap();
    eprintln!("Created document");

    let document_provider = Rc::new(DocumentProvider::create(document).unwrap());

    eprintln!("Building internal renderer");
    // let internal_renderer =
    //     InternalTemplateRenderer::build(document_provider.clone(), None).expect("error creating internal renderer");
    let factory = FACTORY.lock().unwrap();
    let internal_renderer =
        factory.build(document_provider.clone(), None).expect("error creating internal renderer");
    eprintln!("Rendering body");
    let body = internal_renderer.render().expect("error rendering body");

    let output = CString::new(body).expect("error creating CString for output");
    output.into_raw()
}
