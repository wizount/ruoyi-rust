use actix_web::{get, HttpRequest, post, put, Responder, web};
use permit_lib::has_permit;

use crate::config::global_variables::{ADMIN_NAME};
use crate::domain::dto::{ UserAddDTO, UserPageDTO, UserRoleAuthQueryDTO, UserUpdateDTO};
use crate::domain::table::SysUser;
use crate::domain::vo::{ PageVO, RespJson, RespVO};
use crate::service::CONTEXT;
use crate::web_data::get_user_name;

#[get("/user/list")]
#[has_permit("system:user:query")]
pub async fn page(arg: web::Query<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.page(&arg.0).await;
    return PageVO::from_result(&vo).resp_json();
}

//前端打开用户添加时，查询岗位和角色列表
#[get("/user/")]
#[has_permit("system:user:query")]
pub async fn before_add(arg: web::Query<UserPageDTO>) -> impl Responder {
    let mut res = RespJson::success_info("操作成功");
    res.insert("roles".to_string(), serde_json::json!(CONTEXT.sys_role_service.finds_all().await.unwrap()));//fixme

    res.insert("posts".to_string(), serde_json::json!(CONTEXT.sys_post_service.finds_all().await.unwrap()));//fixme
    res.resp_json()
}

#[post("/user")]
#[has_permit("system:user:add")]
pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let role_ids = arg.role_ids.clone().unwrap();
    let mut data = SysUser::from(arg.0);
    let user_id = data.user_id.clone().unwrap();
    data.create_by = Some(get_user_name());
    let rows_affected = CONTEXT.sys_user_service.add(data).await.unwrap();
    if rows_affected > 0 {
        if role_ids.len() > 0 {
            CONTEXT.sys_user_role_service.add_user_roles(&user_id, &role_ids);
        }
        //todo 增加post
    }

    return RespVO::from_result(&Ok(rows_affected)).resp_json();
}


//用户编辑，需要查询post和role列表
#[get("/user/{user_id}")]
#[has_permit("system:user:query")]
pub async fn detail(user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    let user = CONTEXT.sys_user_service.detail(&user_id).await.unwrap();

    let mut res = RespJson::success_info("操作成功");

    let role_ids: Vec<String> = CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap()
        .into_iter().map(|r| r.role_id.unwrap()).collect();

    res.insert("data".to_string(), serde_json::json!(user));

    res.insert("posts".to_string(), serde_json::json!(CONTEXT.sys_post_service.finds_all().await.unwrap()));
    res.insert("roleIds".to_string(), serde_json::json!(role_ids));
    res.insert("roles".to_string(), serde_json::json!(CONTEXT.sys_role_service.finds_all().await.unwrap()));

    return res.resp_json();
}

#[put("/user")]
#[has_permit("system:user:edit")]
pub async fn update(arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.update(arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

#[get("/user/{user_id}")]
#[has_permit("system:user:remove")]
pub async fn remove(user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    let vo = CONTEXT
        .sys_user_service
        .remove(&user_id)
        .await;
    return RespVO::from_result(&vo).resp_json();
}


#[get("/user/deptTree")]
#[has_permit("system:user:query")]
pub async fn get_dept_tree() -> impl Responder {
    let dept_tree = CONTEXT.sys_dept_service.get_dept_tree(&get_user_name()).await;
    return RespVO::from_result(&dept_tree).resp_json();
}

#[put("/user/authRole")]
#[has_permit("system:user:query")]
pub async fn set_auth_roles(arg: web::Query<UserRoleAuthQueryDTO>) -> impl Responder {
    let s = arg.role_ids.clone().unwrap_or_default();
    let role_ids: Vec<String> = s.split(",").map(|s| s.to_string()).collect();
    CONTEXT.sys_user_role_service.reset_through_user_id(&arg.user_id.clone().unwrap_or_default(), &role_ids).await;
    RespVO::<u64>::from_success_info("更新成功！").resp_json()
}

#[get("/user/authRole/{user_id}")]
#[has_permit("system:user:query")]
pub async fn get_auth_roles(user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut user = CONTEXT.sys_user_service.detail(&user_id).await.unwrap();
    user.roles = Some(CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap());
    let roles = CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap();
    let filter_roles = match get_user_name().eq(ADMIN_NAME) {
        true => {
            roles
        }
        false => {
            roles.into_iter().filter(|r| r.admin).collect::<Vec<_>>()
        }
    };

    let mut res = RespJson::success_info("操作成功");
    res.insert("user".to_string(), serde_json::json!(user));
    res.insert("roles".to_string(), serde_json::json!(filter_roles));
    return res.resp_json();
}


// AjaxResult ajax = AjaxResult.success();
// SysUser user = userService.selectUserById(userId);
// List<SysRole> roles = roleService.selectRolesByUserId(userId);
// ajax.put("user", user);
// ajax.put("roles", SysUser.isAdmin(userId) ? roles : roles.stream().filter(r -> !r.isAdmin()).collect(Collectors.toList()));
// return ajax;
// }


#[put("/user/resetPwd")]
#[has_permit("system:user:resetPwd")]
pub async fn reset_pwd(arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let user_id = arg.0.user_id.unwrap();
    let user = CONTEXT.sys_user_service.find(&user_id).await.unwrap();
    if user.is_none() {
        return RespVO::<u64>::from_error_info(500, "找不到此用户").resp_json();
    }
    let user = user.unwrap();
    if user.user_name.eq(&Some(ADMIN_NAME.to_string())) {
        return RespVO::<u64>::from_error_info(500, "不允许操作超级管理员用户").resp_json();
    }
    //todo 查看datascope  userService.checkUserDataScope(user.getUserId());

    //user.password = Some(PasswordEncoder::encode(arg.password.as_ref().unwrap()));

    ///   user.setUpdateBy(getUsername());


    //  CONTEXT.sys_user_service.update(&user_id).await.unwrap();
    RespVO::<u64>::from_success_info("更新成功！").resp_json()
}

//更改用户当前状态
#[put("/user/changeStatus")]
#[has_permit("")]
pub async fn change_status(req: HttpRequest, arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let user_id = arg.0.user_id.unwrap();
    let user = CONTEXT.sys_user_service.find(&user_id).await.unwrap();
    if user.is_none() {
        return RespVO::<u64>::from_error_info(500, "找不到此用户").resp_json();
    }
    let user = user.unwrap();
    if user.user_name.eq(&Some(ADMIN_NAME.to_string())) {
        return RespVO::<u64>::from_error_info(500, "不允许操作超级管理员用户").resp_json();
    }
    let vo = CONTEXT.sys_user_service.update_status(
        arg.0.status.unwrap(), &user_id).await;
    return RespVO::from_result(&vo).resp_json();
}