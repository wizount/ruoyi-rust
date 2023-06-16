use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};
use crate::domain::dto::{RoleAddDTO, RoleUpdateDTO};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleMenuAddDTO {
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,

}

impl From<RoleMenuAddDTO> for RoleAddDTO {
    fn from(arg: RoleMenuAddDTO) -> Self {
        Self {
            role_name: arg.name,
            role_key: None,
            role_sort: None,
            data_scope: None,
            menu_check_strictly: None,
            dept_check_strictly: None,
            menu_ids: None,
            status: None,
            remark: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleMenuUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    //菜单id集合
    pub menu_ids: Vec<u64>,
}

impl From<SysRoleMenuUpdateDTO> for RoleUpdateDTO {
    fn from(arg: SysRoleMenuUpdateDTO) -> Self {
        Self {
            role_id: arg.id,
            role_name: arg.name,
            role_key: None,
            role_sort: None,
            data_scope: None,
            menu_check_strictly: None,
            dept_check_strictly: None,
            menu_ids: None,
            status: None,
            remark: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleMenuPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&SysRoleMenuPageDTO> for PageRequest {
    fn from(arg: &SysRoleMenuPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
