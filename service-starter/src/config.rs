use crate::service::service_discover::ServiceDiscoverConfig;
use common::yaml_global_config;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionConfig {
    pub expire_time: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    pub listen: String,
    pub workers: i16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MysqlConfig {
    pub url: String,
    pub pool_size: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaoConfig {
    pub admin_db: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BusinessConfig {
    pub wx_auth: WxAuthConfig,
    pub wx_pay: WxPayConfig,
    pub sms: SmsServiceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WxAuthConfig {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WxPayConfig {
    pub call_back_base_url: String,
    pub app_id: String,
    pub app_secret: String,
    //商户ID
    pub mch_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmsServiceConfig {
    pub secret_id: String,
    pub secret_key: String,
    pub sdk_app_id: String,
    pub template: String,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitcherConfig {
    pub wx_auth_mode: u8,
    pub sms_mode: u8,
    pub wx_pay_mode: u8,
    pub vc_check: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub mysql: MysqlConfig,
    pub session: SessionConfig,
    pub dao: DaoConfig,
    pub service_discover: ServiceDiscoverConfig,
    pub business: BusinessConfig,
    pub switcher: SwitcherConfig,
}

yaml_global_config!(Config);
