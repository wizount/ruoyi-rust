use crate::domain::table::{SysLogininfor};
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysLogininforVO {
    pub info_id: Option<String>,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<char>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
}

impl From<SysLogininfor> for SysLogininforVO {
    fn from(arg: SysLogininfor) -> Self {
        Self {
            info_id: arg.info_id,
            user_name: arg.user_name,
            ipaddr: arg.ipaddr,
            login_location: arg.login_location,
            browser: arg.browser,
            os: arg.os,
            status: arg.status,
            msg: arg.msg,
            login_time: arg.login_time
        }
    }
}
