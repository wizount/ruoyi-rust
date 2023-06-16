use rbatis::rbdc::datetime::DateTime;

///Permission Menu Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysMenu {
    pub menu_id: Option<u64>,
    pub menu_name: Option<String>,
    //父id(可空)
    pub parent_id: Option<u64>,
    //顺序
    pub order_num: Option<u32>,
    //前端-菜单路径
    pub path: Option<String>,
    //组件路径
    pub component: Option<String>,
    //组件路径
    pub query: Option<String>,
    //是否为外链
    pub is_frame: Option<char>,
    //是否缓存
    pub is_cache: Option<char>,
    //菜单类型
    pub menu_type: Option<char>,
    //菜单可见
    pub visible: Option<char>,
    //菜单状态
    pub status: Option<char>,
    //权限标识
    pub perms: Option<String>,
    //图标
    pub icon: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRole {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<char>,
    pub dept_check_strictly: Option<char>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

///Role menu relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRoleMenu {
    //角色id
    pub role_id: Option<String>,
    //菜单id
    pub menu_id: Option<u64>,
}

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    // pub user_type: Option<String>, fixme 目前没有用上
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
}

///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDept {
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
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,

}

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictType {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictTypeSimple {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictData {
    pub dict_code: Option<String>,
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
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysConfig {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysNotice {
    pub notice_id: Option<String>,
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<char>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysLogininfor {
    pub info_id: Option<String>,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<char>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPost {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_time: Option<DateTime>,
}
