use crate::domain::table::{SysUser, SysUserRole};
use crate::util::password_encoder::PasswordEncoder;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAddDTO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<char>,
    pub password: Option<String>,
    pub remark: Option<String>,
    pub role_ids: Option<Vec<String>>,
    pub post_ids: Option<Vec<String>>,
}

impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            user_id: ObjectId::new().to_string().into(),
            dept_id: arg.dept_id,
            user_name: arg.user_name.clone(),
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            status: STATUS_NORMAL.into(),
            del_flag: DEL_FLAG_NORMAL.into(),
            login_ip: None,
            login_date: None,
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
pub struct UserUpdateDTO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<char>,
    pub status: Option<char>,
    pub remark: Option<String>,
    pub role_ids: Option<Vec<String>>,
    pub post_ids: Option<Vec<String>>,
}

impl From<UserUpdateDTO> for SysUser {
    fn from(arg: UserUpdateDTO) -> Self {
        SysUser {
            user_id: arg.user_id,
            dept_id: arg.dept_id,
            user_name: arg.user_name,
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: None,
            status: arg.status,
            del_flag: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub status: Option<String>,
    pub dept_id: Option<String>,
}

impl From<&UserPageDTO> for PageRequest {
    fn from(arg: &UserPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&UserRolePageDTO> for UserPageDTO {
    fn from(arg: &UserRolePageDTO) -> Self {
        Self {
            page_no: arg.page_no.clone(),
            page_size: arg.page_size.clone(),
            user_name: arg.user_name.clone(),
            phonenumber: None,
            status: None,
            dept_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleDTO {
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

impl From<UserRoleDTO> for SysUserRole {
    fn from(arg: UserRoleDTO) -> Self {
        SysUserRole {
            user_id: arg.user_id,
            role_id: arg.role_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsersRoleDTO {
    pub user_ids: String,
    pub role_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub user_name: Option<String>,
    pub name: Option<String>,

}

impl From<&UserRolePageDTO> for PageRequest {
    fn from(arg: &UserRolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleAuthQueryDTO {
    pub user_id: Option<String>,
    pub role_ids: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasswordUpdateDTO {
    pub old_password: Option<String>,
    pub new_password: Option<String>
}
