use crate::Result;
use actix_http::body::Body;
use actix_http::body::{MessageBody, ResponseBody};
use actix_http::http::header::CONTENT_TYPE;
use actix_http::http::{HeaderValue, StatusCode};
use actix_http::ResponseBuilder;
use actix_web::HttpResponse;
use futures_lite::StreamExt;
use serde::Serialize;
use serde_json::Value;

pub fn make_response_body(code: i32, msg: String, others: impl Serialize) -> Result<Vec<u8>> {
    let s = serde_json::to_string(&others)?;
    let mut others: Value = serde_json::from_str(s.as_str())?;
    let mut root = json!({
        "result" : code.to_string(),
        "message": msg,
    });
    match root {
        Value::Object(ref mut v) => match others {
            Value::Object(ref mut v1) => v.append(v1),
            _ => panic!("others should be object!"),
        },
        _ => panic!("impossible!"),
    };
    Ok(serde_json::to_vec(&root)?)
}

pub fn make_error_response(code: i32, msg: String) -> Result<HttpResponse> {
    Ok(ResponseBuilder::new(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; utf-8"),
        )
        .body(Body::from(json!({
            "result" : code.to_string(),
            "message": msg,
        }))))
}

pub fn make_ok_response(others: impl Serialize) -> Result<HttpResponse> {
    Ok(ResponseBuilder::new(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; utf-8"),
        )
        .body(Body::from(make_response_body(0, "操作成功".to_string(), others)?)))
}

pub fn make_common_response() -> Result<HttpResponse> {
    make_error_response(0, "操作成功".to_string())
}

pub fn read_reponse_body<B: MessageBody>(body: ResponseBody<B>) -> Vec<u8> {
    let bytes = futures_lite::future::block_on(async move {
        let mut tmp: Vec<u8> = Vec::new();
        futures_lite::pin!(body);
        loop {
            match body.next().await {
                Some(Ok(bytes)) => {
                    tmp.extend_from_slice(&bytes);
                }
                Some(Err(e)) => {
                    error!("pull response failed: {}", e);
                    break;
                }
                None => break,
            };
        }
        tmp
    });
    bytes
}
