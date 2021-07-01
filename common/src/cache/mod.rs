pub mod redis;

pub fn make_cache_key(domain: &str, key: &str) -> String {
    format!("{}/{}", domain, key)
}
