use std::collections::BTreeMap;

use crate::domain::dto::{UserPageDTO, UserRoleDTO, UserRolePageDTO};
use crate::domain::table::{SysRole, SysUserRole};
use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::{SysMenuVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use rbatis::sql::Page;
use rbs::to_value;


///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg))
            .await?;
        // if arg.resp_set_role.unwrap_or(true) {
        //     let all_role = CONTEXT.sys_role_service.finds_all_map().await?;
        //     let user_ids = rbatis::make_table_field_vec!(&vo.records, id);
        //     let user_roles = SysUserRole::select_in_column(pool!(), "id", &user_ids).await?;
        //     let user_role_map = rbatis::make_table_field_map!(&user_roles, user_id);
        //     let role_ids = rbatis::make_table_field_vec!(&user_roles, role_id);
        //     let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
        //     let roles_map = rbatis::make_table_field_map!(&roles, id);
        //     for mut x in &mut vo.records {
        //         if let Some(user_role) = user_role_map.get(x.id.as_deref().unwrap_or_default()) {
        //             if let Some(role_id) = &user_role.role_id {
        //                 let role = roles_map.get(role_id).cloned();
        //                 x.role = SysRoleVO::from_option(role);
        //                 //查找子集角色
        //                 if let Some(role_vo) = &mut x.role {
        //                     CONTEXT
        //                         .sys_role_service
        //                         .loop_find_childs(role_vo, &all_role);
        //                 }
        //             }
        //         }
        //     }
        // }
        return Ok(vo);
    }

    pub async fn add(&self, arg: UserRoleDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        let user_role = SysUserRole::from(arg);
        self.remove_by_user_id(user_id.as_str()).await?;
        Ok(SysUserRole::insert(pool!(), &user_role).await?.rows_affected)
    }


    pub async fn add_user_roles(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        let rows = role_ids.into_iter().map(|r_id| SysUserRole {
            user_id: user_id.to_string().into(),
            role_id: r_id.to_string().into(),
        }).collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20)
            .await?.rows_affected)
    }

    pub async fn add_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.into_iter().map(|u_id| SysUserRole {
            user_id: u_id.to_string().into(),
            role_id: role_id.to_string().into(),
        }).collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20)
            .await?
            .rows_affected)
    }


    pub async fn remove(&self, user_role: &SysUserRole) -> Result<u64> {
        let res =
            pool!().exec("delete from sys_user_role where user_id=? and role_id=?",
                         vec![to_value!(user_role.user_id.as_ref().unwrap()), to_value!(user_role.role_id.as_ref().unwrap())]).await.unwrap();
        Ok(res.rows_affected)
    }
    pub async fn remove_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.into_iter().map(|u_id| SysUserRole {
            user_id: u_id.to_string().into(),
            role_id: role_id.to_string().into(),
        }).collect::<Vec<_>>();

        let mut cnt = 0;
        for r in rows {
            let res = self.remove(&r).await;
            cnt = cnt + res.unwrap();
        }
        Ok(cnt)
    }
    pub async fn remove_by_role_id(&self, role_id: &String) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.role_id), role_id)
                .await?
                .rows_affected,
        )
    }


    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?
                .rows_affected,
        )
    }


    pub async fn reset_through_user_id(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
            .await?
            .rows_affected;
        self.add_user_roles(user_id, role_ids).await
    }
    pub async fn find_roles_by_user_id(
        &self,
        user_id: &str,
        all_menus: &BTreeMap<u64, SysMenuVO>,
    ) -> Result<Option<Vec<SysRole>>> {
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?;

        let role_ids = &rbatis::make_table_field_vec!(&user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
        Ok(Some(roles))
    }

    //TODO delete
    // pub async fn find_user_role(
    //     &self,
    //     user_id: &str,
    //     all_menus: &BTreeMap<String, SysMenuVO>,
    // ) -> Result<Option<SysRoleVO>> {
    //     if user_id.is_empty() {
    //         return Ok(None);
    //     }
    //     let user_roles =
    //         SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
    //             .await?;
    //
    //     let role_ids = &rbatis::make_table_field_vec!(&user_roles, role_id);
    //     let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
    //     let role_menu_vec = CONTEXT
    //         .sys_role_service
    //         .find_role_menu(&rbatis::make_table_field_vec!(&user_roles, role_id))
    //         .await?;
    //     let mut role_vos = vec![];
    //     for role in roles {
    //         //load res
    //         let mut menus = vec![];
    //         for role_menu in &role_menu_vec {
    //             if role.id.is_some() && role.id.eq(&role_menu.role_id) {
    //                 if let Some(res) = all_menus.get(role_menu.menu_id.as_ref().unwrap_or_def()) {
    //                     menus.push(res.clone());
    //                 }
    //             }
    //         }
    //         let mut vo = SysRoleVO::from(role);
    //         vo.menu_ids = CONTEXT.sys_menu_service.make_menu_ids(&menus);
    //         vo.menus = menus;
    //         role_vos.push(vo);
    //     }
    //     if role_vos.is_empty() {
    //         return Ok(None);
    //     } else {
    //         return Ok(Some(role_vos[0].clone()));
    //     }
    // }
}
