use crate::domain::dto::{ConfigAddDTO, ConfigUpdateDTO, ConfigPageDTO};
use crate::domain::table::SysConfig;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder};
use permit_lib::has_permit;
use actix_web::HttpRequest;

#[get("/config/list")]
#[has_permit("system:config:query")]
pub async fn page(page: web::Query<ConfigPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_config_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/config/{config_id}")]
#[has_permit("system:config:query")]
pub async fn detail(config_id: web::Path<String>) -> impl Responder {
    let config_id = config_id.into_inner();
    let config_vo = CONTEXT.sys_config_service.detail(&config_id).await;
    RespVO::from_result(&config_vo).resp_json()
}


#[post("/config")]
#[has_permit("system:config:add")]
pub async fn add(arg: web::Json<ConfigAddDTO>) -> impl Responder {
    let mut data = SysConfig::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    let res = SysConfig::from(data);
    let data = CONTEXT.sys_config_service.add(&res).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/config")]
#[has_permit("system:config:edit")]
pub async fn update(arg: web::Json<ConfigUpdateDTO>) -> impl Responder {
    let mut data = SysConfig::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_config_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/config/{config_id}")]
#[has_permit("system:config:remove")]
pub async fn remove(config_id: web::Path<String>) -> impl Responder {
    let config_id = config_id.into_inner();
    let data = CONTEXT
        .sys_config_service
        .remove(&config_id)
        .await;
    RespVO::<u64>::judge(data.unwrap(), "".to_string(), "删除失败！".to_string()).resp_json()
}
