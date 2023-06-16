use std::time::Duration;
use actix_web::{get, HttpRequest,  put, Responder, web};

use crate::domain::vo::{RespJson, RespVO};
use crate::service::CONTEXT;
use permit_lib::has_permit;
use crate::domain::dto::{PasswordUpdateDTO, UserUpdateDTO};
use crate::util::password_encoder::PasswordEncoder;

/*
* 用户自身的操作
*/

//的用户信息
#[get("/user/profile")]
#[has_permit("")]
pub async fn profile(req: HttpRequest) -> impl Responder {
    let user_cahce = CONTEXT.sys_user_service.get_user_cache(&req).await.unwrap();
    let mut res = RespJson::success_info("操作成功");

    res.insert("data".to_string(), serde_json::json!(user_cahce.user.unwrap()));
    //todo 职位
    // res.insert("posts".to_string(), serde_json::json!(CONTEXT.sys_post_service.finds_all().await.unwrap()));
    res.insert("roleGroup".to_string(), serde_json::json!(user_cahce.roles.clone().into_iter().map(|r|r.role_name.unwrap()).collect::<Vec<_>>().join(",")));
    return res.resp_json();
}

//用户自行修改用户信息
#[put("/user/profile")]
#[has_permit("")]
pub async fn update_profile(req: HttpRequest, mut arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let mut user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await.unwrap();
    let clone = arg.0.clone();
    arg.0.user_id = user_cache.id.clone().into();
    let vo = CONTEXT.sys_user_service.update(arg.0).await.unwrap();
    if vo > 0 {
        let mut user = user_cache.user.clone().unwrap();
        user.phonenumber = clone.phonenumber;
        user.email = clone.email;
        user_cache.user = user.into();
        CONTEXT.cache_service.set_string_ex(
            &user_cache.token_key,
            &user_cache.to_string(),
            Some(Duration::from_secs(
                CONTEXT.config.token_expired_min * 60
            )),
        ).await;
    }
    return RespVO::from_result(&Ok(vo)).resp_json();
}

//用户自行修改密码
#[put("/user/profile/updatePwd")]
#[has_permit("")]
pub async fn update_pwd(req: HttpRequest, arg: web::Query<PasswordUpdateDTO>) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await.unwrap();
    let user_id = user_cache.id.clone();
    let user = CONTEXT.sys_user_service.find(&user_id).await.unwrap().unwrap();
    let new_password = &arg.new_password;
    let old_password = &arg.old_password;
    if new_password.eq(old_password) {
        return RespVO::<u64>::from_error_info(500, "新密码不能与旧密码相同").resp_json();
    }


    if !PasswordEncoder::verify(user.password.as_ref().unwrap(), &old_password.clone().unwrap()) {
        return RespVO::<u64>::from_error_info(500, "修改密码失败，旧密码错误").resp_json();
    }
    let vo = CONTEXT.sys_user_service.update_password(
        &PasswordEncoder::encode(new_password.as_ref().unwrap()), &user_id).await;
    return RespVO::from_result(&vo).resp_json();
}