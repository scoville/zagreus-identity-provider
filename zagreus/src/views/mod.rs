use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use std::path::Path;
use tera::{Context, Error, ErrorKind, Tera};
use thiserror::Error;

pub mod home;
pub mod invitation;
pub mod invitations;
pub mod login;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let path = Path::new(zagreus_config::env::TEMPLATES_PATH())
            .canonicalize()
            .expect("path to templates couldn't be resolved")
            .join("**/*.html");

        let path = path.as_os_str().to_str().expect("path couldn't be joined");

        let tera = match Tera::new(path) {
            Ok(tera) => tera,
            Err(error) => {
                eprintln!("Parsing error(s): {}", error);

                ::std::process::exit(1);
            }
        };

        tera
    };
}

#[derive(Error, Debug)]
#[error("template couldn't be rendered")]
struct TemplateError;

impl ResponseError for TemplateError {}

pub struct HtmlTemplate<'a, T> {
    filepath: &'a str,
    template: T,
}

impl<'a, T> HtmlTemplate<'a, T>
where
    T: Serialize,
{
    fn new(filepath: &'a str, template: T) -> Self {
        HtmlTemplate { filepath, template }
    }
}

impl<'a, T> Responder for HtmlTemplate<'a, T>
where
    T: Serialize,
{
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let context = match Context::from_serialize(self.template).map_err(|_| TemplateError) {
            Ok(context) => context,
            Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let html_string = match TEMPLATES.render(self.filepath, &context) {
            Ok(html_string) => html_string,
            Err(Error {
                kind: ErrorKind::TemplateNotFound(_),
                ..
            }) => return HttpResponse::new(StatusCode::NOT_FOUND),
            Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        };

        HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(html_string)
    }
}
