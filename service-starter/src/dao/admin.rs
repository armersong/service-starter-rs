use crate::config::MysqlConfig;
use crate::dao::base::MysqlDao;
use crate::model::po::admin::{AdminPo, CustomerServicePo};
use crate::Result;
use mysql_async::prelude::Queryable;
use std::ops::Deref;
use std::sync::Arc;

pub struct AdminDao(Arc<MysqlDao>);

impl Clone for AdminDao {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Deref for AdminDao {
    type Target = MysqlDao;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AdminDao {
    pub fn new(cfg: &MysqlConfig, db_name: &str) -> Result<Self> {
        Ok(Self(Arc::new(MysqlDao::new(cfg, db_name))))
    }

    pub async fn find(&self, sqn: u32) -> Result<Option<AdminPo>> {
        let mut conn = self.conn().await?;
        let res = conn.exec_first(r#"SELECT sqn, account, pass, status,name,
        sex, mobile, email,icon,remark, DATE_FORMAT(ctime, "%Y-%m-%d %T") AS ctime
        FROM admin WHERE sqn=:sqn"#,
                                  params!{
                        "sqn" => sqn,
                    }).await?;
        Ok(res)
    }

    pub async fn find_by_account(&self, account: &str) -> Result<Option<AdminPo>> {
        let mut conn = self.conn().await?;
        let res = conn.exec_first(r#"SELECT sqn, account, pass, status,name,
        sex, mobile, email,icon,remark, DATE_FORMAT(ctime, "%Y-%m-%d %T") AS ctime
        FROM admin WHERE account=:account"#,
                                  params!{
                        "account" => account,
                    }).await?;
        Ok(res)
    }

    pub async fn find_customer_service(&self) -> Result<Option<CustomerServicePo>> {
        let mut conn = self.conn().await?;
        let res = conn.exec_first(r#"SELECT sqn, wechat, mobile, address FROM service ORDER BY sqn DESC"#,
                                  (),).await?;
        Ok(res)
    }
}
