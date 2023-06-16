use crate::domain::table::SysDictType;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDictTypeVO {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<SysDictType> for SysDictTypeVO {
    fn from(arg: SysDictType) -> Self {
        Self {
            dict_id: arg.dict_id,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}

impl SysDictTypeVO {}
