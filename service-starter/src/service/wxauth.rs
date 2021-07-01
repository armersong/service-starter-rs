use crate::config::Config;
use crate::prelude::errcode::RC_FAIL;
use crate::service::service_discover::{ServiceDiscover, AUTHORIZOR_SERVICE};
use crate::{Error, ErrorKind, Result};
use actix_web::client::Client;
use actix_web::http::StatusCode;
use common::{get_json_integer, get_json_str};
use openssl::aes::{aes_ige, AesKey};
use openssl::base64::decode_block;
use openssl::symm::{decrypt, Cipher, Crypter, Mode};
use serde_json::{Map, Value};
use std::sync::Arc;
use crate::model::doo::wxauth::{WxAuthRequest, WxAuthResponse};
use crate::model::po::admin::Sex;

pub struct WxAuthService(Arc<WxAuthServiceInner>);

impl WxAuthService {
    pub fn new() -> Result<Self> {
        Ok(Self(Arc::new(WxAuthServiceInner {})))
    }

    pub async fn auth(&self, req: WxAuthRequest) -> Result<WxAuthResponse> {
        match Config::get().switcher.wx_auth_mode {
            1 => self.auth_proxy(req).await,
            2 => self.auth_debug(req).await,
            0 | _ => self.auth_local(req).await,
        }
    }

    pub async fn auth_debug(&self, req: WxAuthRequest) -> Result<WxAuthResponse> {
        Ok(WxAuthResponse {
            open_id: "".to_string(),
            union_id: "".to_string(),
            url_head: "https://thirdwx.qlogo.cn/mmopen/vi_32/KMELdYwnYP7DQuoPYhd1icNnrtlERh4OJXO9MpiaAapavC5Az2nIRdU0BcNkJt4yE5LJXnKUsYaSrs57RqGo5N0A/132".to_string(),
            usr_nick: "".to_string(),
            usr_sex: Sex::Male,
        })
    }
    pub async fn auth_proxy(&self, req: WxAuthRequest) -> Result<WxAuthResponse> {
        let cfg = &Config::get().business.wx_auth;
        unimplemented!()
    }

    pub async fn auth_local(&self, req: WxAuthRequest) -> Result<WxAuthResponse> {
        let cfg = &Config::get().business.wx_auth;
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", cfg.app_id,cfg.app_secret, req.ath_code.as_str());
        info!("wx auth url {}", url);
        let mut client = Client::default();
        let mut resp = client.get(url.as_str()).send().await.map_err(|e| {
            Error::from(ErrorKind::Network(format!("request {} failed: {:?}", url, e)))
        })?;
        if resp.status() != StatusCode::OK {
            return Err(ErrorKind::Custom(
                RC_FAIL,
                format!("获取微信unionid失败: {}", resp.status().as_str()),
            )
            .into());
        }
        let rbody = resp
            .body()
            .await
            .map_err(|e| Error::from(ErrorKind::Network(format!("read body failed: {:?}", e))))?;
        info!("wx auth response {:?}", rbody);
        let json: Map<String, Value> = serde_json::from_slice(&rbody)?;
        let errmsg = get_json_str(&json, "errmsg").unwrap_or("ok".to_string());
        let errcode = get_json_integer(&json, "errcode").unwrap_or(0);
        if errcode != 0 {
            return Err(ErrorKind::Custom(
                RC_FAIL,
                format!("Request unionid error: {} {}", errcode, errmsg),
            )
            .into());
        }
        let open_id = get_json_str(&json, "openid")?;
        let session_key = get_json_str(&json, "session_key")?;
        info!(
            "wx get union id: code {} open_id {} session_key {}",
            req.ath_code, open_id, session_key
        );
        let de_str =
            Self::decryt(session_key.as_str(), req.cryption.as_str(), req.vector.as_str())?;
        info!("decrypt str {}", de_str);
        let info: Map<String, Value> = serde_json::from_str(de_str.as_str())?;
        info!("decrypt json {:?}", info);
        let nick = get_json_str(&info, "nickName")?;
        let head_url = get_json_str(&info, "avatarUrl")?;
        let sex = get_json_integer(&info, "gender")?;

        Ok(WxAuthResponse {
            open_id: open_id.clone(),
            union_id: open_id,
            url_head: head_url,
            usr_nick: nick,
            usr_sex: if sex == 1 { Sex::Male } else { Sex::Female },
        })
    }

    fn decryt(sesion_key: &str, crypt_data: &str, iv: &str) -> Result<String> {
        debug!("decrypt session_key {} crypt_data {} iv {}", sesion_key, crypt_data, iv);
        let sk = decode_block(sesion_key).map_err(|e| {
            Error::from(ErrorKind::Custom(
                RC_FAIL,
                format!("decode session_key {} failed", sesion_key),
            ))
        })?;
        let cd = decode_block(crypt_data).map_err(|e| {
            Error::from(ErrorKind::Custom(
                RC_FAIL,
                format!("decode crypt data {} failed", crypt_data),
            ))
        })?;
        let iv_ = decode_block(iv).map_err(|e| {
            Error::from(ErrorKind::Custom(RC_FAIL, format!("decode iv {} failed", iv)))
        })?;
        debug!("sk {} ecrypt data {} iv {}", sk.len(), cd.len(), iv_.len());
        let data =
            decrypt(Cipher::aes_128_cbc(), sk.as_slice(), Some(iv_.as_slice()), cd.as_slice())
                .map_err(|e| {
                    Error::from(ErrorKind::Custom(
                        RC_FAIL,
                        format!("decrypt {} failed: {}", crypt_data, e),
                    ))
                })?;
        Ok(String::from_utf8_lossy(data.as_slice()).to_string())
    }
}

struct WxAuthServiceInner {}

impl Clone for WxAuthService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::model::doo::WxAuthRequest;
    use crate::prelude::errcode::RC_FAIL;
    use crate::service::service_discover::ServiceDiscover;
    use crate::service::wxauth::WxAuthService;
    use crate::{Error, ErrorKind, Result};
    use serde_json::Value;
    use std::path::Path;
    use crate::model::doo::wxauth::WxAuthRequest;

    #[actix_web::rt::test]
    async fn test_wx() {
        assert!(init().is_ok());
        test_wx_login().await;
        // assert!(test_wx_login().await.is_ok());
    }

    fn init() -> Result<()> {
        let base = Path::new("config");
        Config::load_from_file(base.join("config.yml").display().to_string().as_str())?;
        ServiceDiscover::init_once(&Config::get().service_discover)?;
        Ok(())
    }

    async fn test_wx_login() -> Result<()> {
        let service = WxAuthService::new()?;
        let res = service
            .auth(WxAuthRequest {
                ath_code: "1234567".to_string(),
                apl_idntfir: "".to_string(),
                spl_type: 2,
                vector: "".to_string(),
                cryption: "".to_string(),
            })
            .await?;
        Ok(())
    }

    #[test]
    fn test_wx_decrypt() -> Result<()> {
        let session_key = "tiihtNczf5v6AKRyjwEUhQ==";
        let encrypted_data = "CiyLU1Aw2KjvrjMdj8YKliAjtP4gsMZMQmRzooG2xrDcvSnxIMXFufNstNGTyaGS9uT5geRa0W4oTOb1WT7fJlAC+oNPdbB+3hVbJSRgv+4lGOETKUQz6OYStslQ142dNCuabNPGBzlooOmB231qMM85d2/fV6ChevvXvQP8Hkue1poOFtnEtpyxVLW1zAo6/1Xx1COxFvrc2d7UL/lmHInNlxuacJXwu0fjpXfz/YqYzBIBzD6WUfTIF9GRHpOn/Hz7saL8xz+W//FRAUid1OksQaQx4CMs8LOddcQhULW4ucetDf96JcR3g0gfRK4PC7E/r7Z6xNrXd2UIeorGj5Ef7b1pJAYB6Y5anaHqZ9J6nKEBvB4DnNLIVWSgARns/8wR2SiRS7MNACwTyrGvt9ts8p12PKFdlqYTopNHR1Vf7XjfhQlVsAJdNiKdYmYVoKlaRv85IfVunYzO0IKXsyl7JCUjCpoG20f0a04COwfneQAGGwd5oa+T8yO5hzuyDb/XcxxmK01EpqOyuxINew==";
        let iv = "r7BXXKkLb8qrSNn05n0qiA==";
        let service = WxAuthService::new()?;
        let s = WxAuthService::decryt(session_key, encrypted_data, iv).map_err(|e| {
            Error::from(ErrorKind::Custom(RC_FAIL, format!("decrypt failed: {}", e)))
        })?;
        println!("decrypt {}", s);
        let json: Value = serde_json::from_str(s.as_str())?;
        println!("json {:?}", json);
        Ok(())
    }
}
