
use std::env;
use std::rc::Rc;
use std::path::Path;

use dotenv::dotenv;

use error::*;
use traits::*;
use expressions::*;
use ast::*;
use objects::*;
use input::*;


#[derive(Debug)]
enum TemplateSource<'a> {
    TemplatePathSource(&'a Path),
    TemplateSource(Rc<Template>),
    DocumentSource(Document)
}

impl<'a, P: AsRef<Path>> From<&'a P> for TemplateSource<'a> {
    fn from(source: &'a P) -> Self {
        TemplateSource::TemplatePathSource(source.as_ref())
    }
}

// impl<'a> From<&'a AsRef<Template>> for TemplateSource<'a> {
//     fn from(source: &'a AsRef<Template>) -> Self {
//         TemplateSource::TemplateSource(source.as_ref())
//     }    
// }

impl<'a> From<Rc<Template>> for TemplateSource<'a> {
    fn from(source: Rc<Template>) -> Self {
        TemplateSource::TemplateSource(source)
    }
}

impl From<Document> for TemplateSource<'static> {
    fn from(doc: Document) -> Self {
        TemplateSource::DocumentSource(doc)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentProvider(Rc<Document>);

impl Default for DocumentProvider {
    fn default() -> Self {
        let source_path = env::var_os("DEFAULT_PAGE")
            .unwrap_or_else(|| "./res/tests/app/todomvc/app.ism".into());
        let source_path = Path::new(&source_path);

        let res = from_source(&source_path);

        if let Err(ref e) = res {
            eprintln!("Error when processing document: {:?}\n", e);
            // let bt = Backtrace::new();
            // eprintln!("Backtrace:\n{:?}", bt);

            panic!("Cannot process document");
        };

        let template = res.unwrap();

        DocumentProvider(Rc::new(template))
    }
}

impl DocumentProvider {
    pub fn doc<'a>(&'a self) -> &'a Document { self.0.as_ref() }
}

// fn create_document(template: &Template) -> DocumentProcessingResult<Document> {
//     // let mut ctx = Context::default();
//     // let mut processing = ProcessDocument::from_template(&template);
//     // processing.process_document(&mut ctx).unwrap();
//     // processing.into()

//     TryProcessFrom::try_processing_from(template)
// }

// fn create_document(template: &Template) -> DocumentProcessingResult<Document> {
//     TryProcessFrom::try_processing_from(template)
// }

fn from_source<'a, S: Into<TemplateSource<'a>>>(source: S) -> DocumentProcessingResult<Document> {
    let source = source.into();

    match source {
        TemplateSource::TemplatePathSource(ref source_path) => {
            let template = parser::parse_file(source_path)?;
            let mut ctx: DefaultProcessingContext<ProcessedExpression> = DefaultProcessingContext::for_template(Rc::new(template.clone()));
            TryProcessFrom::try_process_from(&template, &mut ctx)
        }

        TemplateSource::TemplateSource(template) => {
            let mut ctx: DefaultProcessingContext<ProcessedExpression> = DefaultProcessingContext::for_template(template.clone());
            TryProcessFrom::try_process_from(template.as_ref(), &mut ctx)
        }

        TemplateSource::DocumentSource(document) => Ok(document)
    }
}