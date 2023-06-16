use crate::domain::dto::{DictTypeAddDTO, DictTypeUpdateDTO, DictTypePageDTO};
use crate::domain::table::SysDictType;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, web, Responder, HttpRequest, delete};
use crate::config::global_variables::STATUS_NORMAL;
use permit_lib::has_permit;

#[get("/dict/type/list")]
#[has_permit("system:dict:query")]
pub async fn page(page: web::Query<DictTypePageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_type_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/dict/type/optionselect")]
#[has_permit("system:dict:query")]
pub async fn optionselect() -> impl Responder {
    let data = CONTEXT.sys_dict_type_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

#[get("/dict/type/{dict_id}")]
#[has_permit("system:dict:query")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let dict_type_vo = CONTEXT.sys_dict_type_service.detail(&dict_id).await;
    RespVO::from_result(&dict_type_vo).resp_json()
}

#[post("/dict/type")]
#[has_permit("system:dict:add")]
pub async fn add(arg: web::Json<DictTypeAddDTO>) -> impl Responder {
    let mut data = SysDictType::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.dict_name.is_none() {
        return RespVO::<u64>::from_error_info(500, "字典名字不能为空!").resp_json();
    }
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_dict_type_service.add(&data).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/dict/type")]
#[has_permit("system:dict:edit")]
pub async fn update(arg: web::Json<DictTypeUpdateDTO>) -> impl Responder {
    let mut data = SysDictType::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_dict_type_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/dict/type/{dict_id}")]
#[has_permit("system:dict:remove")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT.sys_dict_type_service
        .remove(&dict_id).await;
    RespVO::from_result(&data).resp_json()
}
