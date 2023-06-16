use crate::domain::table::SysRoleMenu;
use crate::error::Result;
use crate::pool;

/// Role Menu Service
pub struct SysRoleMenuService {}

impl SysRoleMenuService {
    pub async fn add_role_menus(&self, role_id: String, menu_ids: Vec<u64>) -> Result<u64> {
        let mut sys_role_menu = vec![];
        for menu_id in menu_ids {
            sys_role_menu.push(SysRoleMenu {
                role_id: Some(role_id.clone()),
                menu_id: Some(menu_id),
            });
        }
        Ok(SysRoleMenu::insert_batch(pool!(), &sys_role_menu, 20)
            .await?
            .rows_affected)
    }

    pub async fn add_roles_menu(&self, menu_id: u64, role_ids: Vec<String>) -> Result<u64> {
        let mut sys_role_menus = vec![];
        for role_id in role_ids {
            sys_role_menus.push(SysRoleMenu {
                role_id: Some(role_id.clone()),
                menu_id: Some(menu_id),
            });
        }
        Ok(SysRoleMenu::insert_batch(pool!(), &sys_role_menus, 20)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_menu_id(&self, menu_id: &u64) -> Result<u64> {
        Ok(SysRoleMenu::delete_by_column(pool!(), "menu_id", menu_id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_role_id(&self, role_id: &String) -> Result<u64> {
        Ok(SysRoleMenu::delete_by_column(pool!(), "role_id", role_id)
            .await?
            .rows_affected)
    }

    pub async fn select_by_role_id(&self, role_id: &String) -> Result<Vec<SysRoleMenu>> {
        Ok(SysRoleMenu::select_by_column(pool!(), "role_id", role_id)
            .await?)
    }
}
