use crate::service::admin::AdminService;
use crate::service::sms::SmsService;
use crate::service::wxauth::WxAuthService;
use crate::service::wxpay::WxPayService;
use crate::Result;

pub struct ServiceBuilder {}

struct ServiceInstances {
    wxauth: WxAuthService,
    sms: SmsService,
    wxpay: WxPayService,
    admin: AdminService,
}

impl_singleton!(ServiceBuilder, ServiceInstances);

impl ServiceBuilder {
    pub fn new() -> ServiceBuilder {
        ServiceBuilder {}
    }

    pub fn build_wxauth(&self) -> WxAuthService {
        Self::sinner().wxauth.clone()
    }
    pub fn build_sms(&self) -> SmsService {
        Self::sinner().sms.clone()
    }
    pub fn build_pay(&self) -> WxPayService {
        Self::sinner().wxpay.clone()
    }
    pub fn build_admin(&self) -> AdminService {
        Self::sinner().admin.clone()
    }

    /// Called by impl_singleton!
    fn do_init() -> Result<ServiceInstances> {
        Ok(ServiceInstances {
            wxauth: WxAuthService::new()?,
            sms: SmsService::new()?,
            wxpay: WxPayService::new()?,
            admin: AdminService::new()?,
        })
    }
}
