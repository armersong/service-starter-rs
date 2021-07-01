#![allow(unused)]

use crate::config::Config;
use common::RedisCache;
use mysql_async::prelude::Queryable;
use std::path::Path;

#[macro_use]
extern crate log;
#[macro_use]
extern crate mysql_async;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate common;

mod config;
mod controller;
#[macro_use]
mod dao;
mod model;
mod prelude;
mod service;

use crate::dao::builder::DaoBuilder;
use crate::service::builder::ServiceBuilder;
use crate::service::service_discover::ServiceDiscover;
use prelude::*;

async fn test_mysql() -> Result<()> {
    let pool = mysql_async::Pool::new(Config::get().mysql.url.as_str());
    let mut conn = pool.get_conn().await?;
    let res: Option<i32> = conn.exec_first("SELECT 1", ()).await?;
    assert!(res.is_some());
    debug!("test_mysql {:?}", res);
    info!("mysql connection ok");
    Ok(())
}

#[actix_web::main]
// #[tokio::main]
async fn main() -> Result<()> {
    let base = Path::new("config");
    common::log::init_log(base.join("log4rs.yml").display().to_string().as_str())?;
    info!("version: {}", env!("CARGO_PKG_VERSION"));
    info!("load config");
    Config::load_from_file(base.join("config.yml").display().to_string().as_str())?;
    info!("test mysql connection");
    test_mysql().await?;
    info!("init redis cache {:?}", Config::get().redis);
    RedisCache::init_singleton_once(
        Config::get().redis.url.as_str(),
        Config::get().redis.pool_size as usize,
    )?;
    info!("init service discover");
    ServiceDiscover::init_once(&Config::get().service_discover)?;
    info!("init daos");
    DaoBuilder::init_singleton_once()?;
    info!("init services");
    ServiceBuilder::init_singleton_once()?;
    info!("init web");
    controller::init_web().await?;
    Ok(())
}
