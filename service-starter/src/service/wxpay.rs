use crate::config::Config;
use crate::model::doo::wxpay::WxPayRequest;
use crate::prelude::errcode::RC_FAIL;
use crate::service::service_discover::{ServiceDiscover, PAY_SERVICE};
use crate::{Error, ErrorKind, Result};
use actix_web::client::Client;
use actix_web::http::StatusCode;
use common::util::now_in_milliseconds;
use common::{get_json_integer, get_json_str};
use rand::{Rng, RngCore};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;

pub struct WxPayService(Arc<WxPayServiceInner>);

impl WxPayService {
    pub fn new() -> Result<Self> {
        Ok(Self(Arc::new(WxPayServiceInner {})))
    }

    pub async fn get_wechat_pay_token(
        &self,
        union_id: &str,
        order_code: &str,
        amount: f32,
        cnt: u32,
    ) -> Result<String> {
        match Config::get().switcher.wx_pay_mode {
            1 => self.get_wechat_pay_token_proxy(union_id, order_code, amount, cnt).await,
            // 2 => self.auth_debug(req).await,
            0 | _ => self.get_wechat_pay_token_local(union_id, order_code, amount, cnt).await,
        }
    }

    fn gen_callback_url() -> String {
        format!(
            "{}/v1/pay/on_wechat_result_callback",
            Config::get().business.wx_pay.call_back_base_url
        )
    }
    // @return 前端支付token
    pub async fn get_wechat_pay_token_proxy(
        &self,
        union_id: &str,
        order_code: &str,
        amount: f32,
        _cnt: u32,
    ) -> Result<String> {
        unimplemented!()
    }

    pub async fn get_wechat_pay_token_local(
        &self,
        union_id: &str,
        order_code: &str,
        amount: f32,
        _cnt: u32,
    ) -> Result<String> {
        let cfg = &Config::get().business.wx_pay;
        let url = "https://api.mch.weixin.qq.com/pay/unifiedorder";
        let total_fee = (amount * 100.0) as u32;
        let mut req = WxLocalPayRequest {
            app_id: cfg.app_id.clone(),
            mch_id: cfg.mch_id.clone(),
            nonce_str: WxLocalPayRequest::gen_nonce(),
            body: "购买".to_string(),
            out_trade_no: order_code.to_string(),
            total_fee,
            spbill_create_ip: "127.0.0.1".to_string(),
            notify_url: Self::gen_callback_url(),
            trade_type: "JSAPI".to_string(),
            open_id: union_id.to_string(),
            sign: "".to_string(),
        };
        let body = req.build(cfg.app_secret.as_str())?;
        info!("body {}", body);
        let mut client = Client::default();
        let mut resp = client.post(url).send_body(body).await.map_err(|e| {
            Error::from(ErrorKind::Network(format!("request {} failed: {:?}", url, e)))
        })?;
        if resp.status() != StatusCode::OK {
            return Err(ErrorKind::Custom(
                RC_FAIL,
                format!("生成预支付凭证失败: {}", resp.status().as_str()),
            )
            .into());
        }
        let rbody =
            String::from_utf8_lossy(&resp.body().await.map_err(|e| {
                Error::from(ErrorKind::Network(format!("read body failed: {:?}", e)))
            })?)
            .to_string();
        info!("response: {}", rbody);
        let res: WxLocalPayResponse = serde_xml_rs::from_str(rbody.as_str()).map_err(|e| {
            Error::from(ErrorKind::Custom(RC_FAIL, format!("deserialize pay xml failed: {:?}", e)))
        })?;
        if res.return_code != "SUCCESS" && res.result_code != "SUCCESS" {
            error!(
                "pay {} failed: {} {} {:?} {:?}",
                url, res.result_code, res.return_code, res.err_code, res.err_code_des
            );
            return Err(ErrorKind::Custom(RC_FAIL, "微信发起支付失败".to_string()).into());
        }
        let mut params: HashMap<&str, &str> = HashMap::new();
        let ts = format!("{}", now_in_milliseconds() / 1000);
        let package = format!("prepay_id={}", res.prepay_id);
        params.insert("appId", res.app_id.as_str());
        params.insert("package", package.as_str());
        params.insert("nonceStr", res.nonce_str.as_str());
        params.insert("timeStamp", ts.as_str());
        params.insert("signType", "MD5");
        let json = json!({
            "appId": res.app_id,
            "package" : package,
            "nonceStr" : res.nonce_str.as_str(),
            "timeStamp" : ts,
            "signType" : "MD5",
            "sign" : WxLocalPayRequest::gen_sign(&params, cfg.app_secret.as_str()),
        });
        let tkn_income = serde_json::to_string(&json)?;
        info!("tkn_income {}", tkn_income);
        Ok(tkn_income)
    }
}

#[derive(Serialize)]
struct WxLocalPayRequest {
    #[serde(rename = "appid")]
    app_id: String,
    mch_id: String,
    nonce_str: String,
    body: String,
    out_trade_no: String,
    total_fee: u32,
    spbill_create_ip: String,
    notify_url: String,
    trade_type: String,
    #[serde(rename = "openid")]
    open_id: String,
    sign: String,
    //optional
    //MD5 default
    // sign_type: String,
    // product_id: String,
    // scene_info: String,
    // detail: String,
}

impl WxLocalPayRequest {
    pub fn gen_nonce() -> String {
        let mut rng = rand::thread_rng();
        let r = rng.next_u32();
        format!("{:08X}", r)
    }

    // @return sign
    pub fn gen_sign(values: &HashMap<&str, &str>, key: &str) -> String {
        let mut v: Vec<(&&str, &&str)> = values.iter().collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        let mut sign_str = String::new();
        for i in 0..v.len() {
            let tmp = if i == 0 {
                format!("{}={}", v[i].0, v[i].1)
            } else {
                format!("&{}={}", v[i].0, v[i].1)
            };
            sign_str.push_str(tmp.as_str());
        }
        sign_str.push_str(format!("&key={}", key).as_str());
        let digest = md5::compute(sign_str.as_bytes());
        let sign = format!("{:X}", digest);
        info!("sign_str: {}, sign {}", sign_str, sign);
        sign
    }

    pub fn build(mut self, key: &str) -> Result<String> {
        let total_fee = format!("{:.2}", self.total_fee);
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("appid", self.app_id.as_str());
        params.insert("mch_id", self.mch_id.as_str());
        params.insert("nonce_str", self.nonce_str.as_str());
        params.insert("body", self.body.as_str());
        params.insert("out_trade_no", self.out_trade_no.as_str());
        params.insert("total_fee", total_fee.as_str());
        params.insert("spbill_create_ip", self.spbill_create_ip.as_str());
        params.insert("notify_url", self.notify_url.as_str());
        params.insert("trade_type", self.trade_type.as_str());
        params.insert("openid", self.open_id.as_str());
        self.sign = Self::gen_sign(&params, key);
        let mut s = String::from("<xml>");
        s.push_str(format!("<appid><![CDATA[{}]]></appid>", self.app_id).as_str());
        s.push_str(format!("<mch_id><![CDATA[{}]]></mch_id>", self.mch_id).as_str());
        s.push_str(format!("<nonce_str><![CDATA[{}]]></nonce_str>", self.nonce_str).as_str());
        s.push_str(format!("<body><![CDATA[{}]]></body>", self.body).as_str());
        s.push_str(
            format!("<out_trade_no><![CDATA[{}]]></out_trade_no>", self.out_trade_no).as_str(),
        );
        s.push_str(format!("<total_fee><![CDATA[{}]]></total_fee>", self.total_fee).as_str());
        s.push_str(
            format!("<spbill_create_ip><![CDATA[{}]]></spbill_create_ip>", self.spbill_create_ip)
                .as_str(),
        );
        s.push_str(format!("<notify_url><![CDATA[{}]]></notify_url>", self.notify_url).as_str());
        s.push_str(format!("<trade_type><![CDATA[{}]]></trade_type>", self.trade_type).as_str());
        s.push_str(format!("<openid><![CDATA[{}]]></openid>", self.open_id).as_str());
        s.push_str(format!("<sign><![CDATA[{}]]></sign>", self.sign).as_str());
        s.push_str("</xml>");
        Ok(s)
    }
}

#[derive(Deserialize)]
struct WxLocalPayResponse {
    pub return_code: String,
    pub return_msg: String,
    #[serde(rename = "appid")]
    pub app_id: String,
    pub mch_id: String,
    pub nonce_str: String,
    pub sign: String,
    pub result_code: String,
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
    pub trade_type: String,
    pub prepay_id: String,
}

struct WxPayServiceInner {}

impl Clone for WxPayService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {}
