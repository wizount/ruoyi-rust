use crate::domain::table::{SysUser};
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use crate::config::global_variables::ADMIN_NAME;
use crate::domain::vo::SysRoleVO;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUserVO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<char>,
    pub avatar: Option<String>,
    pub password: Option<String>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
    pub login_ip: Option<String>,
    pub login_date: Option<DateTime>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    pub admin: bool,
    pub roles: Option<Vec<SysRoleVO>>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            user_id: arg.user_id,
            dept_id: arg.dept_id,
            user_name: arg.user_name.clone(),
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: arg.avatar,
            //屏蔽密码
            password: None,
            status: arg.status,
            del_flag: arg.del_flag,
            login_ip: arg.login_ip,
            login_date: arg.login_date,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
            admin: arg.user_name.unwrap_or_default().eq(ADMIN_NAME),
            roles: None,
        }
    }
}

