use rbatis::object_id::ObjectId;
use crate::domain::table::SysDictType;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
}

impl From<DictTypePageDTO> for PageRequest {
    fn from(arg: DictTypePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&DictTypePageDTO> for PageRequest {
    fn from(arg: &DictTypePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeAddDTO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub remark: Option<String>
}

impl From<DictTypeAddDTO> for SysDictType {
    fn from(arg: DictTypeAddDTO) -> Self {
        SysDictType {
            dict_id: ObjectId::new().to_string().into(),
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: Some(crate::web_data::get_user_name()),
            create_time: DateTime::now().set_micro(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeUpdateDTO {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<DictTypeUpdateDTO> for SysDictType {
    fn from(arg: DictTypeUpdateDTO) -> Self {
        SysDictType {
            dict_id: arg.dict_id,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
            remark: arg.remark,
        }
    }
}
