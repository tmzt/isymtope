extern crate dotenv;
#[macro_use]
extern crate quicli;

extern crate isymtope_ast_common;
extern crate isymtope_generate;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

use dotenv::dotenv;
use quicli::prelude::*;

use isymtope_generate::*;

/// Compile Isymtope files to static pages
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "base-url")]
    base_url: String,

    #[structopt(long = "template_path")]
    template_path: Option<String>,

    #[structopt(long = "app_name")]
    app_name: Option<String>,

    #[structopt(long = "output", short = "o")]
    output: String
}

main!(|args: Cli| {
    dotenv().ok();

    let mut template_cache: HashMap<String, DefaultTemplateContext> = Default::default();
    let app_dir: PathBuf = env::var_os("APP_DIR").expect("APP_DIR must be provided").into();

    let app_name = args.app_name.as_ref().map(|s| s.to_owned())
        .unwrap_or_else(|| env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided if --app-name is not supplied").to_string_lossy().to_string());

    let app_root = app_dir.join(&app_name);
    let template_path = args.template_path.as_ref().map(|s| s.to_owned()).unwrap_or_else(|| "/app.ism".to_owned());
    let template_key = format!("[templatePath={}]", template_path);

    let base_url = args.base_url;
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

    let mut f = File::create(args.output)?;
    f.write_all(body.as_bytes())?;

    // eprintln!("{}", body);
});
