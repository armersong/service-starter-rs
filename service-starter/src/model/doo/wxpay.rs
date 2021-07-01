#[derive(Serialize, Debug)]
pub struct WxPayRequest {
    pub result: String,
    pub url_callback: String,
    //0
    pub spl_type: String,
    // 2位小数
    pub amt_total: String,
    pub odr_internal: String,
    pub message: String,
    pub acc_idntfir: String,
}

#[derive(Deserialize, Debug)]
pub struct WxPayResponse {
    pub tkn_income: String,
}
