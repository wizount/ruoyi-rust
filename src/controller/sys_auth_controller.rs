use crate::domain::vo::{JWTToken, RespJson, RespVO};
use crate::service::CONTEXT;
use actix_web::{HttpRequest, Responder, web};
use permit_lib::has_permit;
use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::config::global_variables::LOGIN_SUC;
use crate::domain::dto::SignInDTO;
use crate::token_auth::{get_token};
use crate::web_data::get_user_name;

// ///Check whether the token_auth and path are valid and accessible
// pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
//     let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
//     RespVO::from_result(&r).resp_json()
// }

pub async fn login(req: HttpRequest, arg: web::Json<SignInDTO>) -> impl Responder {
    let token = CONTEXT.sys_user_service.login(&arg.0, &req).await;
    if token.is_err() {
        return RespVO::from_error_result(500, &token).resp_json();
    }
    let token = token.unwrap();
    let mut res = RespJson::success();
    res.insert("token".to_string(), token.into());
    return res.resp_json();
}

pub async fn logout(req: HttpRequest) -> impl Responder {
    let token = get_token(&req);
    if !token.is_empty() {
        let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if claims.is_ok() {
            let login_user_key = claims.unwrap().login_user_key;
            CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(&req, get_user_name(), LOGIN_SUC, "退出成功".to_string())).await;
            CONTEXT.cache_service.del(&format!("{}{}", LOGIN_TOKEN_KEY, login_user_key)).await;
        }
    }
    RespVO::<String>::from_success_info("退出成功!").resp_json()
}

#[has_permit("")]
pub async fn info(req: HttpRequest) -> impl Responder {
    // match crate::token_auth::check_permit(req, "").await {
    //     None => {}
    //     Some(res) => { return res.resp_json(); }
    // }

    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    // let user_data = CONTEXT
    //     .sys_user_service
    //     .get_user_info_by_token(&user_cache)
    //     .await;
    if user_cache.is_err() {
        return RespVO::<u64>::from_error_info(401, "请重新！").resp_json();
    }
    let user_cache = user_cache.unwrap();
    let mut res = RespJson::success();
    res.insert("permissions".to_string(), serde_json::json!(&user_cache.permissions));
    res.insert("user".to_string(), serde_json::json!(&user_cache.user));
    res.insert("roles".to_string(), serde_json::json!(rbatis::make_table_field_vec!(&user_cache.roles , role_key)));
    res.resp_json()
}
