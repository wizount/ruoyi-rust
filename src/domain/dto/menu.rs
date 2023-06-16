use crate::domain::table::SysMenu;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuPageDTO {
    pub menu_name: Option<String>,
    pub status: Option<char>
}



#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuAddDTO {
    #[validate(length(min = 2, max = 10, message = "菜单名称长度在2到10之间"))]
    #[validate(required(message = "菜单"))]
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
    pub remark: Option<String>,
}

impl From<MenuAddDTO> for SysMenu {
    fn from(arg: MenuAddDTO) -> Self {
        SysMenu {
            menu_id: None,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            order_num: arg.order_num,
            path: arg.path,
            component: arg.component,
            query: arg.query,
            is_frame: arg.is_frame,
            is_cache: arg.is_cache,
            menu_type: arg.menu_type,
            visible: arg.visible,
            status: arg.status,
            perms: arg.perms,
            icon: arg.icon,
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
pub struct MenuUpdateDTO {
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
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<MenuUpdateDTO> for SysMenu {
    fn from(arg: MenuUpdateDTO) -> Self {
        SysMenu {
            menu_id: arg.menu_id,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            order_num: arg.order_num,
            path: arg.path,
            component: arg.component,
            query: arg.query,
            is_frame: arg.is_frame,
            is_cache: arg.is_cache,
            menu_type: arg.menu_type,
            visible: arg.visible,
            status: arg.status,
            perms: arg.perms,
            icon: arg.icon,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
            remark: None,
        }
    }
}
