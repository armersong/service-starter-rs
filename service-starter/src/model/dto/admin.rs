use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct AdminLoginRequest {
    #[validate(length(min = 1, max = 64))]
    pub account: String,
    #[validate(length(min = 1, max = 64))]
    pub pass: String,
}

#[derive(Serialize, Debug)]
pub struct AdminLoginResponse {
    pub token: String,
    pub name: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AdminLogoutRequest {
    pub token: String,
}
