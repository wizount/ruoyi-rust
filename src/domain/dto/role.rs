use rbatis::object_id::ObjectId;
use crate::domain::table::{SysRole};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};
use crate::config::global_variables::{DEL_FLAG_NORMAL};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RolePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub status: Option<char>,
    pub create_begin_time: Option<DateTime>,//fixme 根据前端传来数据，还未找到方法。
    pub create_end_time: Option<DateTime>,
}

impl From<&RolePageDTO> for PageRequest {
    fn from(arg: &RolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleAddDTO {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub menu_ids: Option<Vec<u64>>,
    pub status: Option<char>,
    pub remark: Option<String>,
}




impl From<RoleAddDTO> for SysRole {
    fn from(arg: RoleAddDTO) -> Self {
        SysRole {
            role_id: ObjectId::new().to_string().into(),
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: match arg.menu_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            dept_check_strictly:  match arg.dept_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            status: arg.status,
            del_flag: Some(DEL_FLAG_NORMAL),
            create_by: None,
            create_time: DateTime::now().set_micro(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleUpdateDTO {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub menu_ids: Option<Vec<u64>>,
    pub status: Option<char>,
    pub remark: Option<String>,
}


impl From<RoleUpdateDTO> for SysRole {
    fn from(arg: RoleUpdateDTO) -> Self {
        SysRole {
            role_id: arg.role_id,
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: match arg.menu_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            dept_check_strictly:  match arg.dept_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            status: arg.status,
            del_flag: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
            remark: arg.remark,
        }
    }
}

//

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleAuthUserPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub role_id: Option<String>,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>
}

impl From<&RoleAuthUserPageDTO> for PageRequest {
    fn from(arg: &RoleAuthUserPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
