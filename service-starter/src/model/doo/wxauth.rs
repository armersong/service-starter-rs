use crate::model::po::admin::Sex;

#[derive(Serialize, Debug)]
pub struct WxAuthRequest {
    pub ath_code: String,
    pub apl_idntfir: String,
    // 2
    pub spl_type: i8,
    pub vector: String,
    pub cryption: String,
}

#[derive(Deserialize, Debug)]
pub struct WxAuthResponse {
    #[serde(rename = "uni_idntfir")]
    pub open_id: String,
    #[serde(rename = "acc_idntfir")]
    pub union_id: String,
    pub url_head: String,
    pub usr_nick: String,
    pub usr_sex: Sex,
}
