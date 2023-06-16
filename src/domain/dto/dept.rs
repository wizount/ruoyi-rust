use rbatis::object_id::ObjectId;
use crate::domain::table::SysDept;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};

/// dept query DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptQueryDTO {
    pub dept_name: Option<String>,
    pub status: Option<char>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptAddDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl From<DeptAddDTO> for SysDept {
    fn from(arg: DeptAddDTO) -> Self {
        SysDept {
            dept_id: ObjectId::new().to_string().into(),
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: Some(STATUS_NORMAL),
            del_flag: Some(DEL_FLAG_NORMAL),
            create_by: None,
            create_time: DateTime::now().set_micro(0).into(),
            update_by: None,
            update_time: None,

        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptUpdateDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
}

impl From<DeptUpdateDTO> for SysDept {
    fn from(arg: DeptUpdateDTO) -> Self {
        SysDept {
            dept_id:arg.dept_id,
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
        }
    }
}
