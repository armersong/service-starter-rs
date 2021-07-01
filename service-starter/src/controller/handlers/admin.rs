use crate::config::Config;
use crate::controller::appstate::AppState;
use crate::controller::session::{Session, SESSION_KEY_PREFIX_ADMIN};
use crate::errcode;
use crate::model::dto::admin::{AdminLoginRequest, AdminLoginResponse, AdminLogoutRequest};
use crate::model::po::admin::AdminPo;
use crate::prelude::errcode::RC_FAIL;
use crate::service::token::Token;
use crate::{Error, ErrorKind, Result};
use actix_web::{delete, post, web, HttpResponse};
use common::util::now_in_milliseconds;
use common::{make_common_response, make_ok_response};
use crate::service::admin::AdminService;

#[post("/v1/admin/token")]
pub async fn admin_login(
    ctx: web::Data<AppState>,
    dto: web::Json<AdminLoginRequest>,
) -> Result<HttpResponse> {
    debug!("admin_login>>> {:?}", dto);
    let po = ctx.admin.login(&dto).await?;
    Session::clean_relation(SESSION_KEY_PREFIX_ADMIN, po.sqn).await?;

    let token = Token::make_token(po.sqn.to_string());
    info!("token {}", token);
    let s = Session::create(token.as_str(), SESSION_KEY_PREFIX_ADMIN, po.sqn, None).await?;
    info!("new session {:?}", s);
    let result = AdminLoginResponse { token: token, name: po.name };
    debug!("admin_login<<< {:?}", result);
    make_ok_response(&result)
}

#[delete("/v1/admin/token")]
pub async fn admin_logout(
    _ctx: web::Data<AppState>,
    dto: web::Json<AdminLogoutRequest>,
) -> Result<HttpResponse> {
    debug!("admin_logout>>> {:?}", dto);
    let _ = Session::remove(dto.token.as_str(), SESSION_KEY_PREFIX_ADMIN).await;
    debug!("admin_logout<<<");
    make_common_response()
}

pub async fn do_basic_check(
    ctx: &web::Data<AppState>,
    token: &str,
) -> Result<(Session, AdminPo)> {
    let ses = Session::load(token).await?;
    let admin_info = check_admin(&ctx.admin, ses.sqn).await?;
    Ok((ses, admin_info))
}


pub async fn check_admin(admin: &AdminService, sqn: u32) -> Result<AdminPo> {
    Ok(admin
        .get_info(sqn)
        .await?
        .ok_or(Error::from(ErrorKind::Custom(RC_FAIL, "账号不存在".to_string())))?)
}