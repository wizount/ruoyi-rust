mod cache_service;
mod mem_service;
mod redis_service;
mod sys_config_service;
mod sys_dict_type_service;
mod sys_dict_data_service;
mod sys_menu_service;
mod sys_role_menu_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_trash_service;
mod sys_user_role_service;
mod sys_user_service;
mod sys_logininfor_service;
mod sys_dept_service;
mod sys_post_service;
mod sys_notice_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_dict_type_service::*;
pub use sys_dict_data_service::*;
pub use sys_menu_service::*;
pub use sys_role_menu_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_trash_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;
pub use sys_dept_service::*;
pub use sys_logininfor_service::*;
pub use sys_post_service::*;
pub use sys_notice_service::*;


/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rb.clone()
    };
}

#[macro_export]
macro_rules! get_config_value {
    ($arg:expr)=> {
        $crate::service::CONTEXT.sys_config_service.select_config_by_key($arg).await.unwrap_or_default()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
    pub cache_service: CacheService,
    pub sys_menu_service: SysMenuService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_menu_service: SysRoleMenuService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_data_service: SysDictDataService,
    pub sys_config_service: SysConfigService,
    pub sys_dept_service: SysDeptService,
    pub sys_trash_service: SysTrashService,
    pub sys_logininfor_service: SysLogininforService,
    pub sys_post_service: SysPostService,
    pub sys_notice_service: SysNoticeService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        //连接数据库
        println!(
            "[ruoyi_rust] rbatis pool init ({})...",
            self.config.database_url
        );
        self.rb
            .init(MysqlDriver {}, &self.config.database_url)
            .expect("[ruoyi_rust] rbatis pool init fail!");
        log::info!(
            "[ruoyi_rust] rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").status()
        );

        println!("Local:  http://{}",self.config.server_url.replace("0.0.0.0", "127.0.0.1"));
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: crate::domain::init_rbatis(&config),
            cache_service: CacheService::new(&config).unwrap(),
            config,
            sys_menu_service: SysMenuService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_menu_service: SysRoleMenuService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_type_service: SysDictTypeService {},
            sys_dict_data_service: SysDictDataService {},
            sys_config_service: SysConfigService {},
            sys_dept_service: SysDeptService {},
            sys_trash_service: SysTrashService {},
            sys_logininfor_service: SysLogininforService {},
            sys_post_service: SysPostService {},
            sys_notice_service: SysNoticeService{},
        }
    }
}
