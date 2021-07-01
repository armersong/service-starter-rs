use crate::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub const AUTHORIZOR_SERVICE: &str = "AUTHORIZOR";
pub const SMS_SERVICE: &str = "SMS";
pub const PAY_SERVICE: &str = "PAY";

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceDiscoverConfig {
    pub services: HashMap<String, ServiceInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServiceInfo {
    addr: String, // ip: port
}

impl ServiceInfo {
    pub fn select_better(&self) -> &str {
        self.addr.as_str()
    }
}

pub struct ServiceDiscover(Arc<ServiceDiscoverInner>);

impl ServiceDiscover {
    pub fn init_once(cfg: &ServiceDiscoverConfig) -> Result<()> {
        let mut services = HashMap::new();
        for (k, v) in cfg.services.iter() {
            services.insert(k.clone(), ServiceInfo { addr: v.addr.clone() });
        }
        unsafe {
            SERVICE_DISCOVER =
                Some(Self(Arc::new(ServiceDiscoverInner { services: RwLock::new(services) })));
        }
        Ok(())
    }

    pub fn find(&self, name: &str) -> Option<ServiceInfo> {
        let services = self.0.services.read().unwrap();
        services.get(&name.to_string()).map(|v| v.clone())
    }

    pub fn inst() -> &'static ServiceDiscover {
        unsafe {
            if let Some(ref sd) = SERVICE_DISCOVER {
                return sd;
            }
        }
        panic!("ServiceDiscover not inited!");
    }
}

struct ServiceDiscoverInner {
    services: RwLock<HashMap<String, ServiceInfo>>,
}

impl Clone for ServiceDiscover {
    fn clone(&self) -> Self {
        ServiceDiscover(self.0.clone())
    }
}

static mut SERVICE_DISCOVER: Option<ServiceDiscover> = None;
