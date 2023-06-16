use crate::domain::dto::{RoleAddDTO, RoleUpdateDTO, RolePageDTO, RoleAuthUserPageDTO, UserRoleDTO, UsersRoleDTO};
use crate::domain::vo::{PageVO, RespJson, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, web, Responder, HttpRequest, delete};
use crate::domain::table::{SysRole, SysUserRole};
use permit_lib::has_permit;

#[get("/role/list")]
#[has_permit("system:role:query")]
pub async fn page(arg: web::Query<RolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.page(&arg.0).await;
    return PageVO::from_result(&vo).resp_json();
}

#[get("/role/{role_id}")]
#[has_permit("system:role:query")]
pub async fn detail(role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();
    let role_vo = CONTEXT.sys_role_service.detail(&role_id).await;
    RespVO::from_result(&role_vo).resp_json()
}

#[post("/role")]
#[has_permit("system:role:add")]
pub async fn add(arg: web::Json<RoleAddDTO>) -> impl Responder {
    let menu_ids = arg.0.menu_ids.clone().unwrap();
    let mut data = SysRole::from(arg.0);

    data.create_by = Some(crate::web_data::get_user_name());
    let vo = CONTEXT.sys_role_service.add(data, menu_ids).await;
    return RespVO::from_result(&vo).resp_json();
}

#[put("/role")]
#[has_permit("system:role:edit")]
pub async fn update(arg: web::Json<RoleUpdateDTO>) -> impl Responder {
    let menu_ids = arg.0.menu_ids.clone().unwrap();
    let mut data = SysRole::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let vo = CONTEXT.sys_role_service.update(data, menu_ids).await;
    return RespVO::from_result(&vo).resp_json();
}

#[delete("/role/{role_id}")]
#[has_permit("system:role:remove")]
pub async fn remove(role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();
    let vo = CONTEXT.sys_role_service.remove(&role_id).await;
    return RespVO::from_result(&Ok(1)).resp_json();
}

//已分配此角色的用户
#[get("/role/authUser/allocatedList")]
#[has_permit("system:role:query")]
pub async fn auth_user_list(arg: web::Query<RoleAuthUserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.auth_user_list_page(&arg.0).await.unwrap();
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(vo));
    res.resp_json()
}

//未分配此角色的用户
#[get("/role/authUser/unallocatedList")]
#[has_permit("system:role:query")]
pub async fn unallocated_user_list(arg: web::Query<RoleAuthUserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.unallocated_user_list_page(&arg.0).await.unwrap();
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(vo));
    res.resp_json()
}

//取消对某个用户授权
#[put("/role/authUser/cancel")]
#[has_permit("system:role:query")]
pub async fn cancel_user(arg: web::Json<UserRoleDTO>) -> impl Responder {
    let row = CONTEXT.sys_user_role_service.remove(&SysUserRole::from(arg.0)).await.unwrap();
    RespVO::<u64>::judge(row, "取消授权成功。".to_string(), "取消授权失败！".to_string()).resp_json()
}

//对多个用户进行授权
#[put("/role/authUser/selectAll")]
#[has_permit("system:role:query")]
pub async fn auth_user_all(arg: web::Query<UsersRoleDTO>) -> impl Responder {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let res = CONTEXT.sys_user_role_service.add_users_role(&arg.0.role_id, &user_ids).await;
    RespVO::<u64>::judge(res.unwrap(), "批量授权成功。".to_string(), "批量授权失败！".to_string()).resp_json()
}
//对多个用户进行授权
#[put("/role/authUser/cancelAll")]
#[has_permit("system:role:query")]
pub async fn cancel_user_all(arg: web::Query<UsersRoleDTO>) -> impl Responder {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let res = CONTEXT.sys_user_role_service.remove_users_role(&arg.0.role_id, &user_ids).await;
    RespVO::<u64>::judge(res.unwrap(), "批量取消授权成功。".to_string(), "批量取消授权失败！".to_string()).resp_json()
}

#[put("/role/changeStatus")]
#[has_permit("system:role:edit")]
pub async fn change_status(arg: web::Json<RoleUpdateDTO>) -> impl Responder {

    //  roleService.checkRoleAllowed(role);  todo
    //         roleService.checkRoleDataScope(role.getRoleId()); todo
    //         role.setUpdateBy(getUsername());
    let mut data = SysRole::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let res = CONTEXT.sys_role_service.update(data, vec![]).await;
    return RespVO::from_result(&res).resp_json();
}
