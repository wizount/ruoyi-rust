use std::collections::{BTreeMap};
use rbs::to_value;
use crate::config::global_variables::{CHAR_FALSE, INNER_LINK, LAYOUT, PARENT_VIEW, TYPE_DIR, TYPE_MENU, ADMIN_NAME};
use crate::domain::dto::MenuPageDTO;

use crate::domain::table::SysMenu;
use crate::domain::vo::{MenuTreeSelectVO, MetaVO, RouterVO, SysMenuVO, UserCache};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use crate::util::string::capitalize;

const RES_MENU_KEY: &'static str = "sys_menu:all";

/// Menu service
pub struct SysMenuService {}

impl SysMenuService {
    pub async fn query_menu(&self, query: &MenuPageDTO) -> Result<Vec<SysMenuVO>> {
        let res: Vec<SysMenuVO> = SysMenu::query_menu(pool!(), query).await?.into_iter().map(|m| SysMenuVO::from(m)).collect();
        Ok(res)
    }
    /// Find the menu array
    pub async fn all(&self) -> Result<Vec<SysMenuVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysMenu>>>(RES_MENU_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[ruoyi_rust] get from redis:{}", RES_MENU_KEY);
        }
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        return Ok(arr);
    }

    ///menu details
    pub async fn detail(&self, menu_id: &str) -> Result<SysMenuVO> {
        let menu = SysMenu::select_by_column(pool!(), field_name!(SysMenu.menu_id), menu_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", menu_id)))?;
        let menu_vo = SysMenuVO::from(menu);
        return Ok(menu_vo);
    }
    pub async fn add(&self, arg: &SysMenu) -> Result<u64> {
        let result = Ok(SysMenu::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    pub async fn update(&self, arg: &SysMenu) -> Result<u64> {
        let result = SysMenu::update_by_column(pool!(), &arg, "menu_id").await?;
        self.update_cache().await?;
        return Ok(result.rows_affected);
    }

    pub async fn remove(&self, id: &u64) -> Result<u64> {
        let trash = SysMenu::select_by_column(pool!(), "menu_id", id).await?;

        if trash.len() == 1 {
            let count: u64 = pool!()
                .query_decode("select count(1) as count from sys_menu where parent_id =?", vec![to_value!(id)])
                .await
                .unwrap();
            if count > 0 { return Err(Error::from("存在子菜单,不允许删除！")); }
        } else {
            return Err(Error::from(format!("菜单id{}不存在！", id)));
        }
        let num = SysMenu::delete_by_column(pool!(), "menu_id", id)
            .await?
            .rows_affected;
        CONTEXT.sys_trash_service.add("sys_menu", &trash).await?;

        self.update_cache().await?;
        return Ok(num);
    }

    pub async fn get_menu_list_by_user_id(&self, user_id: &String) -> Result<Vec<SysMenuVO>> {
        let res: Option<Vec<SysMenuVO>> = pool!().query_decode("
      select distinct m.menu_id, m.parent_id, m.menu_name, m.path, m.component, m.`query`, m.visible, m.status, ifnull(m.perms,'') as perms, m.is_frame, m.is_cache, m.menu_type, m.icon, m.order_num, m.create_time
		from sys_menu m
		left join sys_role_menu rm on m.menu_id = rm.menu_id
		left join sys_user_role ur on rm.role_id = ur.role_id
		left join sys_role ro on ur.role_id = ro.role_id
		where ur.user_id = ?
		order by m.parent_id, m.order_num
       ", vec![to_value!(user_id)])
            .await.unwrap();
        Ok(res.unwrap())
    }
    // pub async fn get_menu_ids_by_role_id(&self, role_id: &String) -> Result<Vec<u64>> {
    //     let res:Option<Vec<SysMenu>>= pool!().query_decode("
    //     select m.menu_id,m.menu_name
    // 	from sys_menu m
    //     left join sys_role_menu rm on m.menu_id = rm.menu_id
    //     where rm.role_id = ?
    //     and m.menu_id not in (select m.parent_id from sys_menu m inner join sys_role_menu rm on m.menu_id = rm.menu_id and rm.role_id = ?)
    //     order by m.parent_id, m.order_num
    //    ", vec![to_value!(&role_id), to_value!(&role_id)])
    //         .await?;
    //     println!("{:?}",res);
    //     let res = rbatis::make_table_field_vec!(&res.unwrap(), menu_id);
    //     Ok(res)
    // }

    // pub fn make_menu_ids(&self, args: &Vec<SysMenuVO>) -> Vec<u64> {
    //     let mut ids = vec![];
    //     for x in args {
    //         ids.push(x.menu_id.unwrap_or_default());
    //         if let Some(childs) = &x.children {
    //             let child_ids = rbatis::make_table_field_vec!(childs, menu_id);
    //             for child_id in child_ids {
    //                 ids.push(child_id);
    //             }
    //         }
    //     }
    //     ids
    // }


    pub async fn update_cache(&self) -> Result<Vec<SysMenuVO>> {
        let all = SysMenu::select_all_order_num(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_MENU_KEY, &all).await?;
        let mut v = vec![];
        for x in all {
            v.push(x.into());
        }
        return Ok(v);
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<u64, SysMenuVO>> {
        let all = self.all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.menu_id.unwrap_or_default(), x);
        }
        return Ok(result);
    }

    pub fn finds_menu(
        &self,
        ids: &Vec<u64>,
        all_menus: &BTreeMap<u64, SysMenuVO>,
    ) -> Vec<SysMenuVO> {
        let mut res = vec![];
        //filter res id
        for (k, v) in all_menus {
            for x in ids {
                if k == x {
                    res.push(v.clone());
                    break;
                }
            }
        }
        res
    }


    //变成id 和label
    pub fn tree_select(&self, menus: Vec<SysMenuVO>) -> Result<Vec<MenuTreeSelectVO>> {
        let mut d = vec![];
        for menu in menus {
            let mut t = MenuTreeSelectVO {
                id: menu.menu_id,
                label: menu.menu_name,
                children: None,
            };
            let c = menu.children.unwrap();
            if !c.is_empty() {
                t.children = Some(self.tree_select(c)?);
            }
            d.push(t);
        }
        Ok(d)
    }
    ///An menus array with a hierarchy
    pub fn build_menu_tree(&self, all_menus: &Vec<SysMenuVO>) -> Result<Vec<SysMenuVO>> {
        //find tops
        let mut tops = vec![];
        for item in all_menus {
            //parent id null, it is an top menu
            if item.is_parent() {
                tops.push(item.clone());
            }
        }
        //find child
        tops.sort_by(|a, b| a.order_num.cmp(&b.order_num));
        for mut item in &mut tops {
            self.loop_find_children(&mut item, &all_menus);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_children(&self, arg: &mut SysMenuVO, all_menus: &Vec<SysMenuVO>) {
        let mut children = vec![];
        for item in all_menus {
            if !item.is_parent() && item.parent_id == arg.menu_id {
                let mut item = item.clone();
                self.loop_find_children(&mut item, all_menus);
                children.push(item);
            }
        }
        if !children.is_empty() {
            children.sort_by(|a, b| a.order_num.cmp(&b.order_num));
            arg.children = Some(children);
        }
    }

    ///生成菜单
    pub async fn get_routers(&self, user_cache: &UserCache) -> Result<Vec<RouterVO>> {
        let all_menus = self.all().await?;
        let filtered_menus = if user_cache.user_name == ADMIN_NAME {
            all_menus
        } else {
            let mut t = vec![];
            for v in all_menus {
                for x in &user_cache.menu_ids {
                    if &v.menu_id.unwrap_or_default() == x {
                        t.push(v.clone());
                    }
                }
            }
            t
        };

        let menu_tree = self.build_menu_tree(&filtered_menus)?;
        Ok(self.build_routers(&menu_tree))
    }

    fn build_routers(&self, menus: &Vec<SysMenuVO>) -> Vec<RouterVO> {
        let mut routers = vec![];
        for menu in menus {
            let menu_type = menu.menu_type.clone().unwrap_or_default();
            if menu_type != TYPE_DIR && menu_type != TYPE_MENU { continue; }
            let mut router = RouterVO {
                name: Some(self.get_route_name(&menu)),
                path: Some(self.get_router_path(&menu)),
                hidden: Some(menu.visible.clone().unwrap() == CHAR_FALSE),
                redirect: None,
                component: Some(self.get_component(&menu)),
                query: menu.query.clone(),
                always_show: None,
                meta: MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(menu.is_cache.unwrap() == CHAR_FALSE),
                    link: None,
                }.into(),
                children: vec![],
            };
            let c_menus = menu.children.clone().unwrap();
            if c_menus.len() > 0 && menu.menu_type.unwrap() == TYPE_DIR {
                router.always_show = Some(true);
                router.redirect = Some("noRedirect".to_string());
                router.children = self.build_routers(&c_menus);
            } else if menu.is_menu_frame() {
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(capitalize(&menu.path.clone().unwrap())),//大写
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: menu.component.clone(),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(menu.is_cache.clone().unwrap() == CHAR_FALSE),
                        link: menu.path.clone(),
                    }.into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            } else if menu.is_parent() && menu.is_inner_link() {
                router.meta = MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(false),
                    link: None,
                }.into();
                router.path = Some("/".to_string());
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(capitalize(&menu.path.clone().unwrap())),//大写
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: Some(INNER_LINK.to_string()),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(true),
                        link: menu.path.clone(),
                    }.into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            }
            routers.push(router);
        }
        routers
    }
    /**
     * 获取路由名称
     *
     * @param menu 菜单信息
     * @return 路由名称
     */
    fn get_route_name(&self, menu: &SysMenuVO) -> String {
        // 非外链并且是一级目录（类型为目录）
        if menu.is_menu_frame() {
            "".to_string()
        } else {
            capitalize(menu.path.as_deref().unwrap_or_default())
        }
    }
    /**
     * 获取路由地址
     *
     * @param menu 菜单信息
     * @return 路由地址
     */
    fn get_router_path(&self, menu: &SysMenuVO) -> String {
        let mut router_path = menu.path.clone().unwrap();
        // 内链打开外网方式
        if !menu.is_parent() && menu.is_inner_link() {
            // router_path = innerLinkReplaceEach(router_path);
        }
        // 非外链并且是一级目录（类型为目录）

        if menu.is_parent() && menu.menu_type.clone().unwrap() == TYPE_DIR
            && menu.is_frame.unwrap() == CHAR_FALSE {
            router_path = "/".to_string() + &router_path;
        }
        // 非外链并且是一级目录（类型为菜单）
        else if menu.is_menu_frame() {
            router_path = "/".to_string();
        }
        router_path
    }
    /**
     * 获取组件信息
     *
     * @param menu 菜单信息
     * @return 组件信息
     */
    fn get_component(&self, menu: &SysMenuVO) -> String {
        let old_component = menu.component.as_deref().unwrap_or_default();
        let mut component = LAYOUT;
        if old_component.len() > 0 && !menu.is_menu_frame() {
            component = menu.component.as_ref().unwrap();
        } else if old_component.len() == 0 && !menu.is_parent() && menu.is_inner_link()
        {
            component = INNER_LINK;
        } else if old_component.len() == 0 && menu.is_parent_view()
        {
            component = PARENT_VIEW;
        }
        component.to_string()
    }
}
