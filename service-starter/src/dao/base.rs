use crate::config::MysqlConfig;
use crate::{ErrorKind, Result};
use mysql_async::prelude::Queryable;
use mysql_async::Transaction;
use mysql_async::{Conn, Opts, OptsBuilder, Pool, TxOpts};
use std::future::Future;

pub struct MysqlDao {
    pool: Pool,
}

impl MysqlDao {
    pub fn new(cfg: &MysqlConfig, db_name: &str) -> Self {
        let opts = Opts::from(cfg.url.as_str());
        let builder = OptsBuilder::from_opts(opts).db_name(Some(db_name));
        let pool = mysql_async::Pool::new(Opts::from(builder));
        Self { pool }
    }

    pub async fn conn(&self) -> Result<Conn> {
        Ok(self.pool.get_conn().await?)
    }

    pub fn trans_opts(&self) -> TxOpts {
        TxOpts::default()
    }

    pub async fn start_trans<F, R, T>(&self, mut f: F) -> Result<T>
    where
        R: Future<Output = std::result::Result<(Conn, T), (Conn, crate::Error)>>,
        F: FnMut(Conn) -> R,
    {
        let mut conn = self.conn().await?;
        conn.query_drop("START TRANSACTION").await?;
        let res = match f(conn).await {
            Ok((mut conn, res)) => {
                info!("try auto commit!");
                conn.query_drop("COMMIT").await.map_err(|e| error!("commit failed: {:?}", e));
                res
            }
            Err((mut conn, e)) => {
                match e.kind() {
                    ErrorKind::Mysql(e) => {
                        warn!("mysql error, do rollback by connection, maybe!");
                    }
                    _ => {
                        info!("try auto rollback!");
                        conn.query_drop("ROLLBACK")
                            .await
                            .map_err(|e| error!("commit failed: {:?}", e));
                    }
                }
                return Err(e);
            }
        };
        Ok(res)
    }

    pub async fn do_query(&self, conn: &mut Conn) -> Result<()> {
        let _r = conn.query_drop("SELECT 1").await?;
        Ok(())
    }

    pub async fn test_trans(&self) -> Result<()> {
        let res = self
            .start_trans(|mut con| async {
                let r = match self.do_query(&mut con).await {
                    Ok(res) => (con, res),
                    Err(e) => return Err((con, e)),
                };
                Ok(r)
            })
            .await?;
        Ok(res)
    }
}

#[macro_export]
macro_rules! trans_try {
    ($e:expr, $conn: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(($conn, e)),
        }
    };
}
