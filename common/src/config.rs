#[macro_export]
macro_rules! yaml_global_config {
    ($Config:tt) => {
        impl $Config {
            // add code here
            pub fn load_from_file(file_name: &str) -> Result<(), std::io::Error> {
                let f = std::fs::File::open(std::path::Path::new(file_name))?;
                let cfg: $Config = $crate::serde_yaml::from_reader(f).map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))
                })?;
                unsafe {
                    CONFIG = Some(cfg);
                }
                Ok(())
            }

            pub fn get() -> &'static Self {
                unsafe {
                    if let Some(ref s) = CONFIG {
                        return s;
                    }
                }
                panic!("{} not inited!", stringify!($Config));
            }
        }
        static mut CONFIG: Option<$Config> = None;
    };
}
