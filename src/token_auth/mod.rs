use actix_web::{HttpRequest};
use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::config::global_variables::ADMIN_NAME;
use crate::domain::vo::{JWTToken, RespVO, UserCache};
use crate::error::Error;
use crate::service::CONTEXT;

pub const TOKEN_PREFIX: &'static str = "Bearer ";

pub fn get_token(req: &HttpRequest) -> String {
    let mut token = req
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_default();
    if token.starts_with(TOKEN_PREFIX) {
        token = token.replace(TOKEN_PREFIX, "");
    }
    token
}


///Whether the interface is in the whitelist
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    return false;
}

///Check whether the token_auth is valid and has not expired
pub async fn checked_token(token: &str) -> Result<UserCache, Error> {
    //check token_auth alive
    let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match claims {
        Ok(c) => {
            let key = format!("{}{}", LOGIN_TOKEN_KEY, c.login_user_key);
            let user_cache: Result<UserCache, Error> = CONTEXT.cache_service.get_json(&key).await;
            match user_cache {
                Ok(u) => {
                    CONTEXT.cache_service.expire(&key, (CONTEXT.config.token_expired_min * 60) as i32).await;
                    Ok(u)
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

///Permission to check
/// permit_str支持与非 如sys:user:list||sys:user:delete，暂时不实现，只支持一个权限
pub async fn check_auth(user_cache: &UserCache, permit_str: &str) -> Result<(), Error> {
    let permit_str = permit_str.replace("\"", "");
    // println!("permit_str{}", permit_str);
    if permit_str.len() == 0 {
        return Ok(());
    }
    if user_cache.user_name == ADMIN_NAME { return Ok(()); }

    let sys_menu = CONTEXT.sys_menu_service.all().await?;
    //权限校验
    for cache_permission in &user_cache.permissions {
        if cache_permission.eq(&permit_str) {
            return Ok(());
        }
    }
    return Err(crate::error::Error::from(format!("无权限访问{}", permit_str)));
}


//
pub async fn check_permit(req: HttpRequest, permit_str: &str) -> Option<RespVO<u64>> {
    let token = get_token(&req);
    let path = req.path().to_string();
    if is_white_list_api(&path) {
        return None;
    }
    match checked_token(&token).await {
        Ok(data) => {
            match check_auth(&data, permit_str).await {
                Ok(_) => {
                    //刷新过期时间
                    crate::web_data::set_user_name(data.user_name);
                }
                Err(e) => {
                    //仅提示拦截
                    let resp: RespVO<u64> = RespVO {
                        code: 500,
                        msg: Some(e.to_string()),
                        data: None,
                    };
                    return Some(resp);
                    // return Ok(req.into_response(resp.resp_json()));
                }
            }
        }
        Err(e) => {
            //401 http状态码会强制前端退出当前登陆状态
            let resp: RespVO<u64> = RespVO {
                code: 401,
                msg: Some(format!("Unauthorized for:{}", e.to_string())),
                data: None,
            };
            return Some(resp);
        }
    }
    None
}
