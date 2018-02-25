extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quicli;

extern crate isymtope_ast_common;
extern crate isymtope_generate;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Write;

use dotenv::dotenv;
use quicli::prelude::*;

use isymtope_ast_common::*;
use isymtope_generate::*;

/// Compile Isymtope files to static pages
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "base-url")]
    base_url: String,

    #[structopt(long = "output", short = "o")]
    output: String,

    input: String,
}

main!(|args: Cli| {
    dotenv().ok();

    let mut template_cache: HashMap<String, DefaultTemplateContext> = Default::default();
    let app_dir: PathBuf = env::var_os("APP_DIR").expect("APP_DIR must be provided").into();
    let app_name: String = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided").to_string_lossy().to_string();

    let app_root = app_dir.join(&app_name);

    let template_path = args.input;
    // let app_key = format!("[appName={}, templatePath={}]", app_name, &template_path);
    let template_key = format!("[templatePath={}]", template_path);

    // let output_string = TEMPLATE_CACHE.with(|cache| {
    //     let mut cache = cache.lock().unwrap();
    //     let app_context = cache
    //         .entry(app_key.clone())
    //         .or_insert_with(|| DefaultTemplateContext::create(app_dir, app_name, template_path).unwrap());

    //     ""
    // });

    let base_url = args.base_url;
    let template_path = "/app.ism".to_owned();
    let path = "/";

    let template_context = template_cache
        .entry(template_key.clone())
        .or_insert_with(|| DefaultTemplateContext::create(&app_root, &template_path).unwrap());


    let req = TemplateRequestMsg::RenderAppRoute(
        base_url.to_owned(),
        app_name.to_owned(),
        template_path.to_owned(),
        path.to_owned(),
    );

    let response = template_context.handle_msg(req)?;
    let TemplateResponseMsg::RenderComplete(result) = response;
    let body = result.into_inner();

    let mut f = File::create("./test.html")?;
    f.write_all(body.as_bytes())?;

    // eprintln!("{}", body);
});
