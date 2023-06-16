use crate::domain::dto::{DictDataAddDTO, DictDataUpdateDTO, DictDataPageDTO};
use crate::domain::table::SysDictData;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder};
use crate::config::global_variables::STATUS_NORMAL;
use permit_lib::has_permit;
use actix_web::HttpRequest;

#[get("/dict/data/list")]
#[has_permit("system:dict:query")]
pub async fn page(page: web::Query<DictDataPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_data_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/dict/data/{dict_id}")]
#[has_permit("system:dict:query")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let dict_data_vo = CONTEXT.sys_dict_data_service.detail(&dict_id).await;
    RespVO::from_result(&dict_data_vo).resp_json()
}

#[post("/dict/data")]
#[has_permit("system:dict:add")]
pub async fn add(arg: web::Json<DictDataAddDTO>) -> impl Responder {
    let mut data = SysDictData::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_dict_data_service.add(&data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "添加失败！".to_string()).resp_json()
}

#[put("/dict/data")]
#[has_permit("system:dict:edit")]
pub async fn update(arg: web::Json<DictDataUpdateDTO>) -> impl Responder {
    let mut data = SysDictData::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_dict_data_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}

#[delete("/dict/data/{dict_code}")]
#[has_permit("system:dict:remove")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT
        .sys_dict_data_service
        .remove(&dict_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}


#[get("/dict/data/type/{dict_type}")]
#[has_permit("")]
pub async fn get_by_dict_type(dict_type: web::Path<String>) -> impl Responder {
    let dict_type = dict_type.into_inner();
    let dict_data_vo = CONTEXT.sys_dict_data_service.get_by_dict_type(&dict_type).await;
    RespVO::from_result(&dict_data_vo).resp_json()
}