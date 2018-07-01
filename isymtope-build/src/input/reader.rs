use std::env;
use std::fs;
use std::io::{self,  Read};
use std::rc::Rc;
use std::path::Path;

use isymtope_ast_common::*;
use input::*;

#[derive(Debug)]
pub enum TemplateSource<'a> {
    TemplatePathSource(&'a Path),
    TemplateSourceString(&'a str),
    TemplateSource(Rc<Template>),
    DocumentSource(Document),
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

impl<'a> From<&'a str> for TemplateSource<'a> {
    fn from(source: &'a str) -> Self {
        TemplateSource::TemplateSourceString(source)
    }
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultDocumentProvider(Rc<Document>);

impl Default for DefaultDocumentProvider {
    fn default() -> Self {
        let source_path =
            env::var_os("DEFAULT_PAGE").unwrap_or_else(|| "./res/tests/app/todomvc/app.ism".into());
        let source_path = Path::new(&source_path);

        let res = DefaultDocumentProvider::create(&source_path);

        if let Err(ref e) = res {
            eprintln!("Error when processing document: {:?}\n", e);
            // let bt = Backtrace::new();
            // eprintln!("Backtrace:\n{:?}", bt);

            panic!("Cannot process document");
        };

        res.unwrap()
    }
}

impl DefaultDocumentProvider {
    pub fn create<'a, S: Into<TemplateSource<'a>>>(
        source: S,
    ) -> DocumentProcessingResult<DefaultDocumentProvider> {
        let doc = from_source(source)?;
        eprintln!("[provider] document: {:?}", doc);

        Ok(DefaultDocumentProvider(Rc::new(doc)))
    }
}

impl DocumentProvider for DefaultDocumentProvider {
    fn doc(&self) -> &Document {
        self.0.as_ref()
    }
}

fn read_file_as_string(path: &Path) -> io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn from_source<'a, S: Into<TemplateSource<'a>>>(source: S) -> DocumentProcessingResult<Document> {
    let source = source.into();

    match source {
        TemplateSource::TemplatePathSource(ref source_path) => {
            let src = read_file_as_string(source_path)?;
            let template = parser::parse_str(&src)?;
            let mut ctx: DefaultProcessingContext<ProcessedExpression> =
                DefaultProcessingContext::for_template(Rc::new(template.clone()));
            TryProcessFrom::try_process_from(&template, &mut ctx)
        }

        TemplateSource::TemplateSourceString(ref src) => {
            let template = parser::parse_str(src)?;
            let mut ctx: DefaultProcessingContext<ProcessedExpression> =
                DefaultProcessingContext::for_template(Rc::new(template.clone()));
            TryProcessFrom::try_process_from(&template, &mut ctx)
        }

        TemplateSource::TemplateSource(template) => {
            let mut ctx: DefaultProcessingContext<ProcessedExpression> =
                DefaultProcessingContext::for_template(template.clone());
            TryProcessFrom::try_process_from(template.as_ref(), &mut ctx)
        }

        TemplateSource::DocumentSource(document) => Ok(document),
    }
}
