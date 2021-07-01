use crate::config::Config;
use crate::model::dto::admin::{AdminLoginRequest, AdminLoginResponse};
use crate::model::po::admin::{AdminPo, AdminStatus, CustomerServicePo};
use crate::model::po::INVALID_SQN;
use crate::prelude::errcode::RC_FAIL;
use crate::service::admin::AdminService;
use crate::service::builder::ServiceBuilder;
use crate::{Error, ErrorKind, Result};
use common::util::now_in_milliseconds;
use num::ToPrimitive;

impl AdminService {
    pub async fn login(&self, info: &AdminLoginRequest) -> Result<AdminPo> {
        let po = self
            .0
            .admin
            .find_by_account(info.account.as_str())
            .await?
            .ok_or(Error::from(ErrorKind::Custom(RC_FAIL, "账号不存在".to_string())))?;
        match po.status {
            AdminStatus::Normal => {}
            n => {
                return Err(ErrorKind::Custom(RC_FAIL, format!("账号已{}", String::from(n))).into())
            }
        }
        if po.pass != info.pass {
            return Err(Error::from(ErrorKind::Custom(RC_FAIL, "密码错误".to_string())));
        }
        Ok(po)
    }

    pub async fn get_info(&self, sqn: u32) -> Result<Option<AdminPo>> {
        self.0.admin.find(sqn).await
    }

    pub async fn get_customer_service_info(&self) -> Result<CustomerServicePo> {
        let rec = self.0.admin.find_customer_service().await?.unwrap_or(CustomerServicePo {
            sqn: INVALID_SQN,
            wechat: "".to_string(),
            mobile: "".to_string(),
            address: "".to_string(),
        });
        Ok(rec)
    }
}
