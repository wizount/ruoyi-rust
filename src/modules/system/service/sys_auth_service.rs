use crate::config::global_constants::{ADMIN_USERID, LOGIN_FAIL, LOGIN_SUC, STATUS_FORBIDDEN};
use crate::context::CONTEXT;
use crate::error::{Error, Result};
use crate::modules::system::constants::ALL_PERMISSIONS;
use crate::system::domain::dto::SignInDTO;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::CommonRoleVO;
use crate::utils::password_encoder::PasswordEncoder;
use crate::web::extractors::user_agent::UserAgent;
use crate::web::token::auth::UserCache;
use crate::web::token::jwt::JwtClaims;
use crate::{error_info, pool};
use rbatis::rbdc::DateTime;
use std::time::Duration;
use uuid::Uuid;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

pub struct SysAuthService {}

impl SysAuthService {
    //返回token
    pub async fn login(&self, sign_in_dto: &SignInDTO, ip: String, user_agent: UserAgent) -> Result<String> {
        self.is_need_wait_login_ex(&sign_in_dto.username).await?;
        let captcha_enabled = CONTEXT
            .sys_config_service
            .select_captcha_enabled()
            .await
            .unwrap_or(false);
        if captcha_enabled {
            if sign_in_dto.code.is_none() {
                return Err(Error::from("请输入验证码！"));
            }
            let code = sign_in_dto.code.as_deref().unwrap();
            if code.len() != 4 {
                return Err(Error::from("验证码输入不正确！"));
            }
            let uuid = sign_in_dto.uuid.as_deref().unwrap_or_default();
            let code_in_cache = CONTEXT
                .cache_service
                .get_string(&format!("{}{}", REDIS_UUID_CAPTCHA, &uuid))
                .await
                .unwrap_or_default();
            if code_in_cache != code {
                return Err(Error::from("验证码输入不正确！"));
            }
        }
        let user: SysUser = SysUser::select_by_map(pool!(), rbs::value! {"user_name":&sign_in_dto.username})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from("账号不存在或者密码错误!"))?;

        if user.status.eq(&Some(STATUS_FORBIDDEN)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;

        // check pwd
        if !PasswordEncoder::verify(
            &user.password.clone().ok_or_else(|| Error::from("用户数据错误!"))?,
            &sign_in_dto.password,
        ) {
            error = Some(Error::from("账号不存在或者密码错误!"));
        }
        if let Some(err) = error {
            let _ = CONTEXT
                .sys_logininfor_service
                .add_async(
                    ip,
                    user_agent,
                    sign_in_dto.username.clone(),
                    LOGIN_FAIL,
                    err.to_string(),
                )
                .await;
            self.add_retry_login_limit_num(&sign_in_dto.username).await?;
            return Err(err);
        }
        let uuid = Uuid::new_v4().to_string();
        let token = self.add_to_cache_and_build_token(user, &uuid, &ip,user_agent.clone()).await;
        let _ = CONTEXT
            .sys_logininfor_service
            .add_async(
                ip,
                user_agent,
                sign_in_dto.username.clone(),
                LOGIN_SUC,
                "成功".to_string(),
            )
            .await;

        token
    }
    ///is need to wait
    async fn is_need_wait_login_ex(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", REDIS_KEY_RETRY, account))
                .await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT
                    .cache_service
                    .ttl(&format!("{}{}", REDIS_KEY_RETRY, account))
                    .await?;
                if wait_sec > 0 {
                    let mut e = error_info!("req_frequently");
                    e = e.replace("{}", &format!("{}", wait_sec));
                    return Err(Error::from(e));
                }
            }
        }
        Ok(())
    }

    ///Add redis retry record
    async fn add_retry_login_limit_num(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", REDIS_KEY_RETRY, account))
                .await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    &format!("{}{}", REDIS_KEY_RETRY, account),
                    &num.to_string(),
                    Some(Duration::from_secs(CONTEXT.config.login_fail_retry_wait_sec)),
                )
                .await?;
        }
        Ok(())
    }
    //返回token
    async fn add_to_cache_and_build_token(
        &self,
        user: SysUser,
        login_user_key: &str,
        ip: &str,
        user_agent: UserAgent,
    ) -> Result<String> {
        let SysUser {
            user_id,
            user_name,
            last_chn_pwd_time,
            ..
        } = user;
        let user_id = user_id.ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let user_name = user_name.ok_or_else(|| Error::from("错误的用户数据，用户名为空!"))?;

        let dept_id = user.dept_id.clone().unwrap_or_default();

        let dept = CONTEXT.sys_dept_service.get_dept_by_id(&dept_id).await?;

        let mut need_chn_pwd = false;
        let (permissions, menu_ids, roles) =
            if CONTEXT.config.chn_pwd_check && last_chn_pwd_time.is_none_or(|d| d.add(Duration::from_secs(180 * 86400)).before(&DateTime::now())) {
                need_chn_pwd = true;
                (vec![], vec![], vec![])
            } else {
                self.load_menu_role_by_user_id(&user_id).await?
            };
        let user_cache = UserCache {
            user_id: user_id.clone(),
            user_name,
            dept_id: dept_id.clone(),
            dept_name: dept.dept_name.unwrap_or_default(),
            permissions,
            menu_ids,
            roles,
            login_time: crate::Now!(),
            login_user_key: login_user_key.to_string(),
            token_key: crate::web::get_login_user_redis_key(login_user_key),
            need_chn_pwd,
            login_ip: ip.to_string(),
            browser: user_agent.browser,
            os: user_agent.os,
        };
        let jwt_token = JwtClaims {
            login_user_key: login_user_key.to_string(),
            exp: crate::Now!().unix_timestamp_millis() as usize,
        };
        let access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;

        let _ = CONTEXT
            .cache_service
            .set_string_ex(
                &user_cache.token_key,
                &user_cache.to_string(),
                Some(Duration::from_secs(CONTEXT.config.token_expired_min * 60)),
            )
            .await;
        Ok(access_token)
    }
    pub async fn load_menu_role_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<CommonRoleVO>)> {
        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;

        let user_roles = SysUserRole::select_by_user_id_status(pool!(), user_id, '0').await?;

        let role_menu = CONTEXT
            .sys_role_service
            .find_role_menu(&rbatis::table_field_vec!(&user_roles, role_id))
            .await?;
        let menu_ids: Vec<String> = rbatis::table_field_vec!(&role_menu, menu_id)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let menus = CONTEXT.sys_menu_service.finds_menu(&menu_ids, &all_menus);

        let permissions: Vec<String> = if user_id.to_string().eq(&ADMIN_USERID) {
            vec![ALL_PERMISSIONS.to_string()]
        } else {
            rbatis::table_field_vec!(&menus, perms)
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        };
        let role_ids = rbatis::table_field_vec!(&user_roles, role_id);

        let roles = CONTEXT
            .sys_role_service
            .finds(&role_ids)
            .await?
            .into_iter()
            .map(|r| CommonRoleVO::from(r))
            .collect();

        Ok((permissions, menu_ids, roles))
    }
}

pub const REDIS_UUID_CAPTCHA: &'static str = "login_captcha:";
