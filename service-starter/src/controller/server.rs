use actix_web::body::MessageBody;
use actix_web::dev::ResponseBody;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{dev, http, App, HttpServer, Result as WebResult};
use common::{read_reponse_body, Result};

use crate::config::Config;
use crate::controller::appstate::AppState;
use crate::controller::handlers;
use crate::service::builder::ServiceBuilder;

fn render_500<B>(res: dev::ServiceResponse<B>) -> WebResult<ErrorHandlerResponse<B>>
where
    B: MessageBody,
{
    render_error(res, 500, "Server internal error")
}

fn render_404<B>(res: dev::ServiceResponse<B>) -> WebResult<ErrorHandlerResponse<B>>
where
    B: MessageBody,
{
    render_error(res, 404, "Not found")
}

fn render_400<B>(res: dev::ServiceResponse<B>) -> WebResult<ErrorHandlerResponse<B>>
where
    B: MessageBody,
{
    render_error(res, 1, "请求参数错误")
}

fn render_error<B>(
    mut res: dev::ServiceResponse<B>,
    code: i32,
    message: &str,
) -> WebResult<ErrorHandlerResponse<B>>
where
    B: MessageBody,
{
    warn!("code {} error {}", code, message);
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json; charset=utf-8"),
    );
    let sc = res.status();
    let bytes = read_reponse_body(res.take_body());
    let resp_body = String::from_utf8_lossy(bytes.as_slice()).to_string();
    let body = match serde_json::to_string(&json!({
        "result": format!("{}", code),
        "message": format!("{}: {}", sc.to_string(), resp_body),
    })) {
        Ok(s) => s,
        Err(_) => format!("{{ \"result\": \"{}\", \"message\": \"{}\" }}", code, message),
    };
    res = res.map_body::<_, B>(|head, _| {
        head.status = StatusCode::OK;
        ResponseBody::Other(body.into())
    });
    Ok(ErrorHandlerResponse::Response(res))
}

pub async fn init_web() -> Result<()> {
    let cfg = Config::get().clone();
    let builder = ServiceBuilder::new();
    let admin = builder.build_admin();
    let wxauth = builder.build_wxauth();
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                admin: admin.clone(),
                wxauth: wxauth.clone(),
            })
            .wrap(
                DefaultHeaders::new()
                    .header("X-Version", "1")
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
                    .header("Access-Control-Allow-Headers", "x-requested-with,content-type"),
            )
            .wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, render_400))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, render_404))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .service(handlers::echo)
            .service(handlers::admin::admin_login)
            .service(handlers::admin::admin_logout)
    })
    .bind(cfg.web.listen)?
    .workers(cfg.web.workers as usize)
    .run()
    .await?;
    Ok(())
}
