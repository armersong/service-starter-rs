use crate::config::Config;
use crate::dao::admin::AdminDao;
use crate::Result;

pub struct DaoBuilder {}

impl_singleton!(DaoBuilder, Daos);

impl DaoBuilder {
    pub fn new() -> Self {
        DaoBuilder {}
    }

    pub fn build_admin(&self) -> AdminDao {
        Self::sinner().admin.clone()
    }

    /// Called by impl_singleton!
    fn do_init() -> Result<Daos> {
        Ok(Daos {
            admin: AdminDao::new(&Config::get().mysql, Config::get().dao.admin_db.as_str())?,
        })
    }
}

struct Daos {
    admin: AdminDao,
}
