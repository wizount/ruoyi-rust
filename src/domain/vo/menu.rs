use crate::domain::table::SysMenu;
use rbatis::rbdc::types::datetime::DateTime;
use crate::config::global_variables::{CHAR_FALSE, TYPE_DIR, TYPE_MENU};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct  SysMenuVO {
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
    pub create_time: Option<DateTime>,
    pub children: Option<Vec<SysMenuVO>>,
}

impl From<SysMenu> for SysMenuVO {
    fn from(arg: SysMenu) -> Self {
        Self {
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
            create_time: arg.create_time,
            children: vec![].into(),
        }
    }
}

impl SysMenuVO {
    pub fn is_parent(&self) -> bool {
        self.parent_id.unwrap_or_default() == 0
    }
    pub fn is_menu_frame(&self) -> bool {
        self.is_parent() && self.menu_type.clone().unwrap_or_default() == TYPE_MENU && self.is_frame.clone().unwrap_or_default() == CHAR_FALSE
    }
    pub fn is_inner_link(&self) -> bool {
        self.is_frame.clone().unwrap_or_default() == CHAR_FALSE && (self.path.clone().unwrap_or_default().starts_with("http://") || self.path.clone().unwrap_or_default().starts_with("https://"))
    }
    pub fn is_parent_view(&self) -> bool {
        !self.is_parent() && self.menu_type.clone().unwrap_or_default() == TYPE_DIR
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct MenuTreeSelectVO {
    pub id: Option<u64>,
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTreeSelectVO>>,
}