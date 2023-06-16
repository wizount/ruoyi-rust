use crate::domain::table::SysRole;
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysRoleVO {
    pub admin:bool,
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    pub flag:bool
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            admin: arg.role_id.clone().unwrap()=="1",
            role_id: arg.role_id,
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: arg.menu_check_strictly.eq(&Some('1')).into(),
            dept_check_strictly:  arg.dept_check_strictly.eq(&Some('1')).into(),
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
            flag: false,
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<SysRole>) -> Option<SysRoleVO> {
        match arg {
            Some(arg) => Some(SysRoleVO {
                admin: arg.role_id.clone().unwrap()=="1",
                role_id: arg.role_id,
                role_name: arg.role_name,
                role_key: arg.role_key,
                role_sort: arg.role_sort,
                data_scope: arg.data_scope,
                menu_check_strictly: arg.menu_check_strictly.eq(&Some('1')).into(),
                dept_check_strictly:  arg.dept_check_strictly.eq(&Some('1')).into(),
                status: arg.status,
                del_flag: arg.del_flag,
                create_by: arg.create_by,
                create_time: arg.create_time,
                update_by: arg.update_by,
                update_time: arg.update_time,
                remark: arg.remark,
                flag: false,
            }),
            _ => None,
        }
    }
}
