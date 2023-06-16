use rbatis::object_id::{ObjectId};
use crate::domain::table::SysDictData;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub dict_type: Option<String>,
    pub dict_label: Option<String>,
    pub status: Option<char>,
}

impl From<DictDataPageDTO> for PageRequest {
    fn from(arg: DictDataPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&DictDataPageDTO> for PageRequest {
    fn from(arg: &DictDataPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataAddDTO {
    pub dict_sort: Option<u32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub remark: Option<String>,
}
impl From<DictDataAddDTO> for SysDictData {
    fn from(arg: DictDataAddDTO) -> Self {
        SysDictData {
            dict_code:ObjectId::new().to_string().into(),
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataUpdateDTO {
    pub dict_code: Option<String>,
    pub dict_sort: Option<u32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<char>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<DictDataUpdateDTO> for SysDictData {
    fn from(arg: DictDataUpdateDTO) -> Self {
        SysDictData {
            dict_code: arg.dict_code,
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}
