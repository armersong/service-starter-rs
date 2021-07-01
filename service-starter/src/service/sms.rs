use crate::config::Config;
use crate::prelude::errcode::RC_FAIL;
use crate::service::service_discover::{ServiceDiscover, SMS_SERVICE};
use crate::{Error, ErrorKind, Result};
use actix_web::client::Client;
use actix_web::http::StatusCode;
use common::{get_json_integer, get_json_str};
use rand::Rng;
use serde_json::{Map, Value};
use std::sync::Arc;
use tencent_sms::TencentSms;

pub struct SmsService(Arc<SmsServiceInner>);

impl SmsService {
    pub fn new() -> Result<Self> {
        Ok(Self(Arc::new(SmsServiceInner {})))
    }

    pub async fn get_vc(&self, mobile: &str) -> Result<String> {
        match Config::get().switcher.sms_mode {
            1 => self.get_vc_proxy(mobile).await,
            2 => self.get_vc_fake(mobile).await,
            0 | _ => self.get_vc_local(mobile).await,
        }
    }
    pub async fn get_vc_local(&self, mobile: &str) -> Result<String> {
        let cfg = &Config::get().business.sms;
        let vc = Self::gen_sms_code();
        debug!("try to send {} to mobile {}", vc, mobile);
        let resp = TencentSms::new(
            cfg.secret_id.as_str(),
            cfg.secret_key.as_str(),
            cfg.sdk_app_id.as_str(),
            cfg.template.as_str(),
            cfg.signature.as_str(),
            vec![ format!("+86{}", mobile)],
            vec![vc.clone()],
        ).send().await.map_err(|e|{
            error!("send sms to {} failed: {}", mobile, e);
            Error::from(ErrorKind::Custom(RC_FAIL, e.to_string()))
        })?;
        info!("send {} to mobile {} ok: {:?}", vc, mobile, resp);
        Ok(vc)
    }

    pub async fn get_vc_fake(&self, mobile: &str) -> Result<String> {
        Ok(Self::gen_sms_code())
    }

    fn gen_sms_code() -> String {
        let mut rng = rand::thread_rng();
        let res: u32 = rng.gen_range(1..999999);
        format!("{:06}", res)
    }

    /// @return verification code
    pub async fn get_vc_proxy(&self, mobile: &str) -> Result<String> {
        unimplemented!()
    }
}

struct SmsServiceInner {}

impl Clone for SmsService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::prelude::errcode::RC_FAIL;
    use crate::service::service_discover::ServiceDiscover;
    use crate::service::sms::SmsService;
    use crate::{Error, ErrorKind, Result};
    use serde_json::Value;
    use std::path::Path;

    fn init() -> Result<()> {
        let base = Path::new("config");
        Config::load_from_file(base.join("config.yml").display().to_string().as_str())?;
        ServiceDiscover::init_once(&Config::get().service_discover)?;
        Ok(())
    }

    async fn test_get_vc() -> Result<()> {
        let service = SmsService::new()?;
        let res = service.get_vc("13510426885").await?;
        println!("vc {}", res);
        Ok(())
    }

    #[test]
    fn test_gen_vc() {
        for i in 0..1000 {
            let vc = SmsService::gen_sms_code();
            // println!("{}: vc {}", i, vc);
            assert!(vc.len() == 6);
        }
    }
}
