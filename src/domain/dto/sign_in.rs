use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub username: String,
    pub password: String,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    pub code: Option<String>,
    pub uuid: Option<String>,
}
