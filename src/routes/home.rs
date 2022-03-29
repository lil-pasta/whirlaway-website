use crate::startup::AppData;
use actix_web::{get, web, Error, HttpResponse};
use tera::Context;

#[get("/")]
pub async fn home(data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let ctx = Context::new();
    let template = data.template.render("home/home.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(template))
}
