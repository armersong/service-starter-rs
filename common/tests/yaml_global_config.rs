#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    pub listen: String,
    pub workers: i16,
}

use common::yaml_global_config;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub redis: RedisConfig,
}
yaml_global_config!(Config);

#[test]
fn load() {
    let cfg_name = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("config.yml")
        .display()
        .to_string();
    let config = Config::load_from_file(cfg_name.as_str());
    if let Err(e) = config {
        println!("current dir {}, load failed: {}", cfg_name, e);
        assert!(false);
    }
    assert!(!Config::get().redis.url.is_empty());
    assert!(!Config::get().web.listen.is_empty());
    assert_eq!(Config::get().web.workers, 4);
}
