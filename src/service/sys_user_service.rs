use std::time::Duration;

use actix_web::HttpRequest;
use chrono::Local;
use rbatis::rbdc::types::datetime::DateTime;
use rbatis::sql::page::{Page, PageRequest};
use rbs::to_value;
use uuid::Uuid;

use crate::config::cache_variables::{REDIS_UUID_CAPTCHA, LOGIN_TOKEN_KEY};
use crate::config::global_variables::{ADMIN_NAME, ALL_PERMISSIONS, LOGIN_FAIL, LOGIN_SUC, STATUS_FORBIDDEN};
use crate::domain::dto::{SignInDTO, UserPageDTO,  UserUpdateDTO};
use crate::domain::table::{SysUser, SysUserRole};
use crate::domain::vo::{JWTToken, UserCache};
use crate::domain::vo::user::SysUserVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use crate::token_auth::get_token;
use crate::util::options::OptionStringRefUnwrapOrDefault;
use crate::util::password_encoder::PasswordEncoder;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

///Background User Service
pub struct SysUserService {}

impl SysUserService {
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> = SysUser::select_page(
            pool!(),
            &PageRequest::from(arg),
            &arg,
        )
            .await?;
        let page = Page::<SysUserVO>::from(sys_user_page);
        return Ok(page);
    }

    ///user details
    pub async fn detail(&self, user_id: &String) -> Result<SysUserVO> {
        let user = self
            .find(&user_id)
            .await?
            .ok_or_else(|| Error::from(format!("用户:{:?} 不存在！", user_id)))?;
        let user_vo = SysUserVO::from(user);
        //  let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;
        return Ok(user_vo);
    }

    pub async fn find(&self, user_id: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), user_id)
                .await?
                .into_iter()
                .next(),
        )
    }

    pub async fn find_by_account(&self, user_name: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.user_name), user_name)
                .await?
                .into_iter()
                .next(),
        )
    }

    pub async fn add(&self, mut user: SysUser) -> Result<u64> {
        if user.user_name.is_none() || user.user_name.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名id不能为空!"));
        }
        let old_user = self
            .find_by_account(user.user_name.as_ref().unwrap_or_def())
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户:{}已存在!",
                user.user_name.as_ref().unwrap()
            )));
        }
        let mut password = user.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            //默认密码
            password = "123456".to_string();
        }
        user.password = Some(password);

        Ok(SysUser::insert(pool!(), &user).await?.rows_affected)
    }

    //返回token
    pub async fn login(&self, arg: &SignInDTO, req: &HttpRequest) -> Result<String> {
        let start = Local::now().timestamp_millis();
        self.is_need_wait_login_ex().await?;
        if CONTEXT.config.captcha_enabled {
            if arg.code.is_none() {
                return Err(Error::from("请输入验证码！"));
            }
            let code = arg.code.as_deref().unwrap();
            if code.len() != 4 {
                return Err(Error::from("验证码输入不正确！"));
            }
            let uuid = arg.uuid.as_deref().unwrap_or_default();
            let code_in_cache = CONTEXT
                .cache_service.get_string(&format!("{}{}", REDIS_UUID_CAPTCHA, &uuid)).await.unwrap_or_default();
            if code_in_cache != code { return Err(Error::from("验证码输入不正确！")); }
        }
       // println!("验证码{}",Local::now().timestamp_millis()-start);
        let user: Option<SysUser> =
            SysUser::select_by_column(pool!(), field_name!(SysUser.user_name), &arg.username)
                .await?.into_iter().next();
        if user.is_none() { return Err(Error::from(format!("账号:{} 不存在!", arg.username))); }
     //   println!("查找是否有这个账号{}",Local::now().timestamp_millis()-start);
        let user = user.unwrap();
        if user.status.eq(&Some(STATUS_FORBIDDEN)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;

        // check pwd
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            &arg.password,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        //todo 加密时间过长，需要换一个，初步定为 https://github.com/RustCrypto/hashes
     //   println!("密码验证{}",Local::now().timestamp_millis()-start);

        if error.is_some() {
            CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(req, arg.username.clone(), LOGIN_FAIL, error.clone().unwrap().to_string())).await;
            self.add_retry_login_limit_num().await?;
            return Err(error.unwrap());
        }
     //   println!("密码验证后{}",Local::now().timestamp_millis()-start);

        let token = self.get_user_info(&user).await;
      //  println!("Token{}",Local::now().timestamp_millis()-start);
        CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(req, arg.username.clone(), LOGIN_SUC, "成功".to_string())).await;
     //   println!("写入日志{}",Local::now().timestamp_millis()-start);
        return token;
    }

    ///is need to wait
    pub async fn is_need_wait_login_ex(&self) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT.cache_service.ttl(REDIS_KEY_RETRY).await?;
                if wait_sec > 0 {
                    return Err(Error::from(format!(
                        "操作过于频繁，请等待{}秒后重试!",
                        wait_sec
                    )));
                }
            }
        }
        return Ok(());
    }

    ///Add redis retry record
    pub async fn add_retry_login_limit_num(&self) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    REDIS_KEY_RETRY,
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec as u64,
                    )),
                )
                .await?;
        }
        return Ok(());
    }

    pub async fn get_user_info_by_token(&self, user_cache: &UserCache) -> Result<String> {
        let user = SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), &user_cache.id)
            .await?
            .into_iter()
            .next();
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", user_cache.user_name)))?;
        return self.get_user_info(&user).await;
    }
    //返回token
    pub async fn get_user_info(&self, user: &SysUser) -> Result<String> {
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let user_id = user
            .user_id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;


        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;

        let uuid = Uuid::new_v4();

        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), &user_id)
                .await?;
        let role_menu = CONTEXT.sys_role_service
            .find_role_menu(&rbatis::make_table_field_vec!(&user_roles, role_id))
            .await?;
        let menu_ids = rbatis::make_table_field_vec!(&role_menu, menu_id);
        let menus = CONTEXT
            .sys_menu_service.finds_menu(&menu_ids, &all_menus);
        println!("{:?}", menus);
        let permissions: Vec<String> = if user.user_name.as_ref().unwrap().to_string() == ADMIN_NAME {
            vec![ALL_PERMISSIONS.to_string()]
        } else {
            rbatis::make_table_field_vec!(&menus, perms)
        };


        let user_cache = UserCache {
            id: user_id.clone(),
            user_name: user.user_name.as_ref().unwrap().to_string(),
            user: Some(user.clone().into()),
            permissions,
            menu_ids,
            roles: CONTEXT.sys_user_role_service.find_roles_by_user_id(
                &user.user_id.unwrap_or_else(|| {
                    return String::new();
                }),
                &all_menus,
            ).await?.unwrap(),
            login_time: DateTime::now().set_micro(0),

            token_key: format!("{}{}", LOGIN_TOKEN_KEY, &uuid.to_string()),
        };
        let jwt_token = JWTToken {
            login_user_key: uuid.to_string(),
            exp: DateTime::now().set_micro(0).unix_timestamp_millis() as usize,
        };
        let access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;

        CONTEXT.cache_service.set_string_ex(
            &user_cache.token_key,
            &user_cache.to_string(),
            Some(Duration::from_secs(
                CONTEXT.config.token_expired_min * 60
            )),
        ).await;
        return Ok(access_token);
    }

    pub async fn update(&self, arg: UserUpdateDTO) -> Result<u64> {
        let role_ids = arg.role_ids.clone();
        let mut user = SysUser::from(arg);
        //do not update user_name
        user.user_name = None;
        let mut password = None;
        //源密码加密后再存储
        if user.password.is_some() {
            password = Some(PasswordEncoder::encode(user.password.as_ref().unwrap()));
        }
        user.password = password;
        if role_ids.is_some() {
            CONTEXT
                .sys_user_role_service.reset_through_user_id(user.user_id.as_ref().unwrap(), &role_ids.unwrap())
                .await?;
        }
        Ok(
            SysUser::update_by_column(pool!(), &user, field_name!(SysUser.user_id))
                .await?
                .rows_affected,
        )
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        let trash = SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), id).await?;
        let r = SysUser::delete_by_column(pool!(), field_name!(SysUser.user_id), id).await?;
        CONTEXT.sys_trash_service.add("sys_user", &trash).await;
        CONTEXT.sys_user_role_service.remove_by_user_id(id).await?;
        return Ok(r.rows_affected);
    }

    ///Find user-authority hierarchy permissions
    // pub async fn load_level_permission(
    //     &self,
    //     user_id: &str,
    //     all_menus: &BTreeMap<String, SysMenuVO>,
    // ) -> Result<Vec<String>> {
    //     return CONTEXT
    //         .sys_role_service
    //         .find_user_permission(user_id, all_menus)
    //         .await;
    // }
    pub async fn get_user_cache(&self, req: &HttpRequest) -> Result<UserCache> {
        let token = get_token(req);
        self.get_user_cache_by_token(&token).await
    }
    pub async fn get_user_cache_by_token(&self, token: &str) -> Result<UserCache> {
        let token = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if token.is_err() {
            return Err(Error::from("Token失败，请重新！"));
        }
        CONTEXT.cache_service.get_json::<UserCache>(&format!("{}{}", LOGIN_TOKEN_KEY, &token.unwrap().login_user_key)).await
    }

    pub async fn update_password(&self, new_password: &str, user_id: &str) -> Result<u64> {
        let res =
            pool!().exec("update sys_user set password = ? where user_id = ?",
                         vec![to_value!(new_password), to_value!(user_id)]).await.unwrap();
        Ok(res.rows_affected)
    }

    pub async fn update_status(&self, status: char, user_id: &str) -> Result<u64> {
        let res =
            pool!().exec("update sys_user set status = ? where user_id = ?",
                         vec![to_value!(status), to_value!(user_id)]).await.unwrap();
        Ok(res.rows_affected)
    }
}
