use crate::service::admin::AdminService;
use crate::service::wxauth::WxAuthService;

#[derive(Clone)]
pub struct AppState {
    pub admin: AdminService,
    pub wxauth: WxAuthService,
}
