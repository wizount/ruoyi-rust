use rbatis::rbdc::datetime::DateTime;
use crate::domain::table::{SysPost};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysPostVO {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<SysPost> for SysPostVO {
    fn from(arg: SysPost) -> Self {
        Self {
            post_id: arg.post_id,
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark
        }
    }
}


