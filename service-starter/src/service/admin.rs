mod admin;

use std::sync::Arc;

use crate::config::Config;
use crate::dao::admin::AdminDao;
use crate::dao::builder::DaoBuilder;
use crate::Result;

struct AdminServiceInner {
    admin: AdminDao,
}

impl AdminServiceInner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            admin: DaoBuilder::new().build_admin(),
        })
    }
}

pub struct AdminService(Arc<AdminServiceInner>);

impl Clone for AdminService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl AdminService {
    pub fn new() -> Result<Self> {
        Ok(Self(Arc::new(AdminServiceInner::new()?)))
    }
}
