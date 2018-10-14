use chrono::{NaiveDateTime, Utc};

use actix_web::error::*;
use actix_web::fs::*;

use isymtope_ast_common::*;
use isymtope_generate::*;
use super::*;

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct GithubConnection {
    pub id: i32,
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub token: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct StoredApp {
    pub slug: String,
    pub base: AppBase,
    pub created_at: NaiveDateTime,
    pub template_name: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum AppBase {
    Template(String),
    App(String),
}

impl StoredApp {
    // pub fn new(base_app_uuid: Option<&str>, base_app_slug: Option<&str>, template_name: Option<&str>) -> Self {
    //     let uuid = format!("{}", Uuid::new_v4());
    //     let slug = allocate_element_key();
    //     let created_at = Utc::now().naive_utc();
    //     let template_name = template_name.map_or_else(|| slug.to_owned(), |s| s.to_owned());

    //     StoredApp {
    //         uuid: uuid,
    //         slug: slug,
    //         created_at: created_at,
    //         base_app_uuid: base_app_uuid.map(|s| s.to_owned()),
    //         base_app_slug: base_app_slug.map(|s| s.to_owned()),
    //         template_name: template_name,
    //     }
    // }

    // fn create_app_from_template(template_data: &TemplateData) -> Self {
    //     let template_name = template_data.template_name.to_owned();
    //     let template_uuid = template_data.uuid.to_owned();
    //     let template_slug = template_data.slug.to_owned();

    //     Self::create(Some(&template_slug), )
    // }

    pub fn create(slug: &str, base: AppBase, template_name: &str) -> Self {
        // let slug = allocate_element_key();
        let created_at = Utc::now().naive_utc();

        StoredApp {
            base: base,
            slug: slug.to_owned(),
            created_at: created_at,
            template_name: template_name.to_owned(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct NewStoredApp<'a> {
    pub uuid: String,
    pub base_app_uuid: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct TemplateFileItem {
    pub id: String,
    pub path: String,
    pub filetype: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct TemplateData {
    pub template_name: String,
    pub files: Vec<TemplateFileItem>,
}

impl TemplateData {
    pub fn read_file(template_name: &str) -> Result<Self> {
        let path = &*EXAMPLES_DIR
            .join(template_name)
            .join("app.json");

        let mut file = NamedFile::open(&path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        serde_json::from_str(&data)
            .map_err(Error::from)
    }
}
