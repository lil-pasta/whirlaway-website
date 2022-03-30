use actix_web::{get, http::StatusCode, Error, HttpResponse};

#[get("/")]
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/html/whirlaway.html")))
}
