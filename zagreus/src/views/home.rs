use actix_web::{get, Responder};
use serde::Serialize;

use super::HtmlTemplate;

#[derive(Debug, Serialize)]
struct HomeTemplate {}

#[get("/")]
pub async fn home() -> impl Responder {
    HtmlTemplate::new("home.html", HomeTemplate {})
}
