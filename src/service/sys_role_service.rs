use crate::domain::dto::{RolePageDTO, RoleAuthUserPageDTO};
use crate::domain::table::{SysRole, SysRoleMenu, SysUserRole};
use crate::domain::vo::{SysRoleVO, SysUserVO};
use crate::error::{Error, Result};
use crate::pool;
use crate::service::CONTEXT;
use rbatis::sql::{Page, PageRequest};
use crate::domain::mapper::sys_user::{db_auth_user_list, db_unallocated_user_list};

const RES_KEY: &'static str = "sys_role:all";

///Role of service
pub struct SysRoleService {}

impl SysRoleService {
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let data = SysRole::select_page(
            pool!(),
            &PageRequest::from(arg),
            &arg,
        )
            .await?;
        //let all_role = self.finds_all_map().await?;
        let page = Page::<SysRoleVO>::from(data);
        // for mut vo in &mut page.records {
        //     self.loop_find_childs(&mut vo, &all_role);
        // }

        Ok(page)
    }
    ///role details
    pub async fn detail(&self, role_id: &str) -> Result<SysRoleVO> {
        let role = SysRole::select_by_column(pool!(), field_name!(SysRole.role_id), role_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", role_id)))?;
        let role_vo = SysRoleVO::from(role);
        return Ok(role_vo);
    }

    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRole::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        return Ok(all);
    }


    pub async fn add(&self, role: SysRole, menu_ids: Vec<u64>) -> Result<u64> {
        let result = SysRole::insert(pool!(), &role).await?.rows_affected;

        if result > 0 && !menu_ids.is_empty() {
            CONTEXT.sys_role_menu_service.add_role_menus(role.role_id.unwrap(), menu_ids);
        }
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn update(&self, role: SysRole, menu_ids: Vec<u64>) -> Result<u64> {
        let result = SysRole::update_by_column(pool!(), &role, field_name!(SysRole.role_id)).await?.rows_affected;
        if result > 0 {
            let role_id = role.role_id.clone().unwrap();
            CONTEXT.sys_role_menu_service.remove_by_role_id(&role_id).await;
            if !menu_ids.is_empty() {
                CONTEXT.sys_role_menu_service.add_role_menus(role_id, menu_ids).await;
            }
        }
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn remove(&self, id: &String) -> Result<u64> {
        let trash = SysRole::select_by_column(pool!(), field_name!(SysRole.role_id), id).await?;
        let result = SysRole::delete_by_column(pool!(), field_name!(SysRole.role_id), id).await?.rows_affected;
        if result > 0 {
            CONTEXT.sys_role_menu_service.remove_by_role_id(id).await;
        }
        CONTEXT.sys_trash_service.add("sys_role", &trash).await?;
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRole::select_in_column(pool!(), "role_id", ids).await?)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysRoleVO>> {
        let data = SysRole::select_all(pool!()).await?;
        let mut role_vos = vec![];
        for s in data {
            role_vos.push(SysRoleVO::from(s));
        }
        Ok(role_vos)
    }

    //查找所有roles，如果用户包含此权限，则flag=true
    pub async fn finds_roles_by_user_id(&self, user_id: &str) -> Result<Vec<SysRoleVO>> {
        let all = SysRole::select_all(pool!()).await?;
        let mut res = vec![];
        let user_roles = SysUserRole::select_by_column(pool!(), "user_id", user_id).await?;
        for r in all {
            let mut r_vo = SysRoleVO::from(r);

            for ur in &user_roles {
                if r_vo.role_id.eq(&ur.role_id) {
                    r_vo.flag = true;
                }
            }
            res.push(r_vo);
        }

        res.sort_by(|a, b| a.role_sort.cmp(&b.role_sort));

        Ok(res)
    }

    pub async fn find_role_menu(&self, role_ids: &Vec<String>) -> Result<Vec<SysRoleMenu>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleMenu::select_in_column(pool!(), "role_id", role_ids).await?)
    }

    // pub async fn find_user_permission(
    //     &self,
    //     user_id: &str,
    //     all_menus: &BTreeMap<String, SysMenuVO>,
    // ) -> Result<Vec<String>> {
    //     let user_roles =
    //         SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
    //             .await?;
    //     let role_menu = self
    //         .find_role_menu(&rbatis::make_table_field_vec!(&user_roles, role_id))
    //         .await?;
    //     let menus =  CONTEXT
    //         .sys_menu_service.finds_menu(&rbatis::make_table_field_vec!(&role_menu, menu_id), &all_menus);
    //     //
    //     // let menus = CONTEXT
    //     //     .sys_menu_service
    //     //     .finds_layer(&rbatis::make_table_field_vec!(&role_menu, menu_id), &all_menus)
    //     //     .await?;
    //     let permissions = rbatis::make_table_field_vec!(&menus, perms);
    //     return Ok(permissions);
    // }
    //
    // ///Loop to find the parent-child associative relation array
    // pub fn loop_find_childs(&self, arg: &mut SysRoleVO, all: &HashMap<String, SysRole>) {
    //     let mut childs = vec![];
    //     for (key, x) in all {
    //         if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
    //             let mut item = SysRoleVO::from(x.clone());
    //             self.loop_find_childs(&mut item, all);
    //             childs.push(item);
    //         }
    //     }
    //     if !childs.is_empty() {
    //         arg.childs = Some(childs);
    //     }
    // }

    pub async fn auth_user_list_page(&self, arg: &RoleAuthUserPageDTO) -> Result<Vec<SysUserVO>> {
        let res = db_auth_user_list(pool!(), &arg).await?;
        let mut sys_users = vec![];
        for user in res {
            sys_users.push(SysUserVO::from(user));
        }
        Ok(sys_users)
    }
    pub async fn unallocated_user_list_page(&self, arg: &RoleAuthUserPageDTO) -> Result<Vec<SysUserVO>> {
        let res = db_unallocated_user_list(pool!(), &arg).await?;
        let mut sys_users = vec![];
        for user in res {
            sys_users.push(SysUserVO::from(user));
        }
        Ok(sys_users)
    }
}
