use crate::domain::dto::{MenuAddDTO, MenuPageDTO, MenuUpdateDTO};
use crate::domain::table::{SysMenu};
use crate::domain::vo::{RespJson, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder, HttpRequest};
use crate::config::global_variables::ADMIN_NAME;
use permit_lib::has_permit;

#[get("/menu/list")]
#[has_permit("system:menu:query")]
pub async fn query_menu(page: web::Query<MenuPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_menu_service.query_menu(&page.0).await;
    RespVO::from_result(&data).resp_json()
}


//菜单栏生成
#[has_permit("")]
pub async fn routers(req: HttpRequest) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    let data = CONTEXT.sys_menu_service.get_routers(&user_cache.unwrap()).await;
    RespVO::from_result(&data).resp_json()
}

#[get("/menu/{menu_id}")]
#[has_permit("system:menu:query")]
pub async fn detail(menu_id: web::Path<String>) -> impl Responder {
    let menu_id = menu_id.into_inner();
    let menu_vo = CONTEXT.sys_menu_service.detail(&menu_id).await;
    RespVO::from_result(&menu_vo).resp_json()
}

#[post("/menu")]
#[has_permit("system:menu:add")]
pub async fn add(arg: actix_web_validator::Json<MenuAddDTO>) -> impl Responder {
    let mut data = SysMenu::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.path.is_none() {
        data.path = Some("".to_string());
    }
    let data = CONTEXT.sys_menu_service.add(&data).await;
    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

#[put("/menu")]
#[has_permit("system:menu:edit")]
pub async fn update(arg: web::Json<MenuUpdateDTO>) -> impl Responder {
    let mut menu = SysMenu::from(arg.0);
    menu.update_by = Some(crate::web_data::get_user_name());

    let cnt = CONTEXT.sys_menu_service.update(&menu).await;

    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&cnt).resp_json()
}

#[delete("/menu/{menu_id}")]
#[has_permit("system:menu:remove")]
pub async fn remove(menu_id: web::Path<u64>) -> impl Responder {
    let menu_id = menu_id.into_inner();
    let data = CONTEXT.sys_menu_service
        .remove(&menu_id).await;
    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

#[get("/menu/treeselect")]
#[has_permit("system:menu:query")]
pub async fn treeselect(req: HttpRequest) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await.unwrap();
    let menus = if user_cache.user_name == ADMIN_NAME {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(&user_cache.id).await
    };
    let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(&menus.unwrap()).unwrap();
    let menu_select = CONTEXT.sys_menu_service.tree_select(menu_tree);
    RespVO::from_result(&menu_select).resp_json()
}

#[get("/menu/roleMenuTreeselect/{role_id}")]
#[has_permit("system:menu:query")]
pub async fn role_menu_treeselect(req: HttpRequest, role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();

    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await.unwrap();
    let menus = if user_cache.user_name == ADMIN_NAME {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(&user_cache.id).await
    };
    let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(&menus.unwrap()).unwrap();
    let menu_select = CONTEXT.sys_menu_service.tree_select(menu_tree);

    let checked_keys = CONTEXT.sys_role_menu_service.select_by_role_id(&role_id).await.unwrap().into_iter().map(|m| m.menu_id.unwrap()).collect::<Vec<_>>();

    let mut res = RespJson::success();
    res.insert("checkedKeys".to_string(), serde_json::json!(checked_keys));
    res.insert("menus".to_string(), serde_json::json!(menu_select.unwrap()));
    res.resp_json()
}