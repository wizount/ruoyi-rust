use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::config::global_variables::ADMIN_NAME;
use crate::domain::vo::{JWTToken, UserCache};
use crate::service::CONTEXT;

pub struct Auth;

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
pub async fn checked_token(token: &str, path: &str) -> Result<UserCache, crate::error::Error> {
    //check token_auth alive
    let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match claims {
        Ok(c) => {
            let key = format!("{}{}", LOGIN_TOKEN_KEY, c.login_user_key);
            let user_cache: Result<UserCache, crate::error::Error> = CONTEXT.cache_service.get_json(&key).await;
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


