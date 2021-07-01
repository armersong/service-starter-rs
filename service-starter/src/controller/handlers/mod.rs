use crate::Result;
use actix_web::{get, HttpResponse};
use common::make_ok_response;

pub mod admin;

#[get("/echo")]
pub async fn echo() -> Result<HttpResponse> {
    Ok(make_ok_response(json!({}))?)
}
