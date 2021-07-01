pub fn init_log(filename: &str) -> Result<(), std::io::Error> {
    log4rs::init_file(filename, Default::default()).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("read log4rs.yml failed: {}", e),
        )
    })
}
