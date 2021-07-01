use std::time::SystemTime;

/// hide some mobile number
/// 13712345678 -> 137****5678
pub fn mask_mobile(s: String) -> String {
    if s.len() < 11 {
        "****".to_string()
    } else {
        let mut src = s;
        let bytes = unsafe { src.as_bytes_mut() };
        let pos = bytes.len() - 8;
        for i in 0..4 {
            bytes[pos + i] = b'*';
        }
        src
    }
}

pub fn now_in_milliseconds() -> u64 {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    duration.as_secs() * 1000 + duration.subsec_millis() as u64
}

///    for init: call init_singleton_once!
///    implement do_init() that is called by init_singleton_once
///    use common::Result;
///    struct DaoBuilder{}
///    impl_singleton!(DaoBuilder, Daos);
///    struct Daos {
///         cleaner: CleanerDao,
///         vendor: VendorDao,
///    };
//     fn do_init() -> Result<Daos> {
//         Ok(Daos{
//             cleaner: CleanerDao::new(&Config::get().mysql, Config::get().dao.cleaner_db.as_str())?,
//             vendor: VendorDao::new(&Config::get().mysql, Config::get().dao.vendor_db.as_str())?,
//         })
//     }
#[macro_export]
macro_rules! impl_singleton {
    ($Singleton:tt, $Inner: tt) => {
        impl $Singleton {
            pub fn init_singleton_once() -> $crate::Result<()> {
                unsafe {
                    if let Some(ref d) = SINGLETON_INNER {
                        panic!("Singleton has been inited!");
                    } else {
                        SINGLETON_INNER = Some(Self::do_init()?);
                        Ok(())
                    }
                }
            }

            fn sinner() -> &'static $Inner {
                unsafe {
                    if let Some(ref d) = SINGLETON_INNER {
                        d
                    } else {
                        panic!("Singleton not inited!");
                    }
                }
            }
        }
        static mut SINGLETON_INNER: Option<$Inner> = None;
    };
}

#[cfg(test)]
mod tests {
    use crate::util::mask_mobile;

    #[test]
    fn test_mask_mobile() {
        assert_eq!(mask_mobile("13700001234".to_string()), "137****1234");
        assert_eq!(mask_mobile("1370001234".to_string()), "****");
    }
}
