#![feature(wasm_import_memory, panic_handler, std_panic)]
#![wasm_import_memory]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use(eprint, eprintln)]
extern crate wasm_log;

extern crate isymtope_ast_common;
extern crate isymtope_build;
extern crate isymtope_generate;

use std::collections::HashMap;
use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;
use std::sync::Mutex;

use isymtope_ast_common::*;
use isymtope_build::*;
use isymtope_generate::*;

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

// thread_local!(
//     static TEMPLATE_CACHE: HashMap<String, DefaultTemplateContext> = Default::default();
// );

fn parse_template(src: &str) -> IsymtopeGenerateResult<DocumentProvider> {
    eprintln!("Parsing document");
    let template = Rc::new(parser::parse_str(&src)?);
    eprintln!("Parsed document");

    // Create document provider
    let mut ctx: DefaultProcessingContext<ProcessedExpression> =
        DefaultProcessingContext::for_template(template.clone());
    eprintln!("Created processing context");
    let document: Document = TryProcessFrom::try_process_from(template.as_ref(), &mut ctx)?;
    eprintln!("Created document");

    let document_provider = DocumentProvider::create(document).unwrap();
    eprintln!("Created document provider");

    Ok(document_provider)
}

#[no_mangle]
pub extern "C" fn compile_template(
    src: *mut c_char,
    app_name: *mut c_char,
    base_url: *mut c_char,
    template_path: *mut c_char,
    path: *mut c_char,
) -> *mut c_char {
    // wasm_log::wasm_log_init();
    // wasm_log::wasm_install_panic_hook();
    // panic::set_hook(Box::new(|panic_info| eprintln!("panic occurred: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap())));

    let src_cstr = unsafe { CString::from_raw(src) };
    let src = src_cstr.to_string_lossy();

    let app_name_cstr = unsafe { CString::from_raw(app_name) };
    let app_name = app_name_cstr.to_string_lossy();

    let base_url_cstr = unsafe { CString::from_raw(base_url) };
    let base_url = base_url_cstr.to_string_lossy();
    eprintln!("Base url {}", base_url);

    let template_path_cstr = unsafe { CString::from_raw(template_path) };
    let template_path = template_path_cstr.to_string_lossy();
    eprintln!("Template path {}", base_url);

    let path_cstr = unsafe { CString::from_raw(path) };
    let path = path_cstr.to_string_lossy();
    eprintln!("Path {}", base_url);

    // Get or cache template context
    let template_key = format!("[templatePath={}]", template_path);
    // let template_context = TEMPLATE_CACHE
    //     .entry(template_key.clone())
    //     .or_insert_with(|| DefaultTemplateContext::new(Rc::new(parse_template(&src).unwrap())));

    let mut template_context = DefaultTemplateContext::new(Rc::new(parse_template(&src).unwrap()));

    let req = TemplateRequestMsg::RenderAppRoute(
        base_url.to_string(),
        app_name.to_string(),
        template_path.to_string(),
        path.to_string(),
    );

    let response = template_context.handle_msg(req).unwrap();
    let TemplateResponseMsg::RenderComplete(result) = response;
    let body = result.into_inner();

    let output = CString::new(body).expect("error creating CString for output");
    output.into_raw()
}
