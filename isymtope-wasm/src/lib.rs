#![feature(wasm_import_memory)]
#![wasm_import_memory]

extern crate isymtope_build;

use std::rc::Rc;

use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

use isymtope_build::error::*;
use isymtope_build::input::*;
use isymtope_build::output::*;


// #[doc(hidden)]
// #[no_mangle]
// pub extern "C" fn __web_malloc( size: usize ) -> *mut u8 {
//     let mut buffer = Vec::with_capacity( size );
//     let ptr = buffer.as_mut_ptr();
//     mem::forget( buffer );
//     ptr
// }

// #[doc(hidden)]
// #[no_mangle]
// pub extern "C" fn __web_free( ptr: *mut u8, capacity: usize ) {
//     unsafe  {
//         let _ = Vec::from_raw_parts( ptr, 0, capacity );
//     }
// }

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

// #[inline(never)]
// fn template_source_to_html(template_source: &str) -> DocumentProcessingResult<String> {
//     let document_provider = DocumentProvider::create(template_source)?;
//     let internal_renderer = InternalTemplateRenderer::build(Rc::new(document_provider), None)?;
//     let body = internal_renderer.render()?;

//     Ok(body)
// }

// #[no_mangle]
// pub extern "C" fn compile_template_source_to_html(data: *mut c_char) -> *mut c_char {
//     unsafe {
//         let data = CStr::from_ptr(data);
//         let src = data.to_string_lossy().to_owned();
//         // let body = template_source_to_html(&src).unwrap();

//         let body = String::from("test");
//         let s = CString::new(body).unwrap();
//         s.into_raw()
//     }
// }