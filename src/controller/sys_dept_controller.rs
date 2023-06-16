use crate::domain::dto::{DeptAddDTO, DeptUpdateDTO, DeptQueryDTO};
use crate::domain::table::SysDept;
use crate::domain::vo::{ RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder};
use permit_lib::has_permit;
use actix_web::HttpRequest;
use crate::config::global_variables::STATUS_NORMAL;

#[get("/dept/list")]
#[has_permit("system:dept:query")]
pub async fn page(query: web::Query<DeptQueryDTO>) -> impl Responder {
    let data = CONTEXT.sys_dept_service.all(&query).await;
    RespVO::from_result(&data).resp_json()
}


#[get("/dept/{dept_id}")]
#[has_permit("system:dept:query")]
pub async fn detail(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let dept_vo = CONTEXT.sys_dept_service.detail(&dept_id).await;
    RespVO::from_result(&dept_vo).resp_json()
}


#[post("/dept")]
#[has_permit("system:dept:add")]
pub async fn add(arg: web::Json<DeptAddDTO>) -> impl Responder {
    let mut data = SysDept::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_dept_service.add(&data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "添加失败！".to_string()).resp_json()
}

#[put("/dept")]
#[has_permit("system:dept:edit")]
pub async fn update(arg: web::Json<DeptUpdateDTO>) -> impl Responder {
    let mut data = SysDept::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_dept_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}

#[delete("/dept/{dept_id}")]
#[has_permit("system:dept:remove")]
pub async fn remove(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let data = CONTEXT
        .sys_dept_service
        .remove(&dept_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
