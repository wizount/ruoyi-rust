use crate::domain::dto::{NoticeAddDTO, NoticeUpdateDTO, NoticePageDTO};
use crate::domain::table::SysNotice;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder};
use permit_lib::has_permit;
use actix_web::HttpRequest;

#[get("/notice/list")]
#[has_permit("system:notice:query")]
pub async fn page(page: web::Query<NoticePageDTO>) -> impl Responder {
    let data = CONTEXT.sys_notice_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/notice/{notice_id}")]
#[has_permit("system:notice:query")]
pub async fn detail(notice_id: web::Path<String>) -> impl Responder {
    let notice_id = notice_id.into_inner();
    let notice_vo = CONTEXT.sys_notice_service.detail(&notice_id).await;
    RespVO::from_result(&notice_vo).resp_json()
}


#[post("/notice")]
#[has_permit("system:notice:add")]
pub async fn add(arg: web::Json<NoticeAddDTO>) -> impl Responder {
    let mut data = SysNotice::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    let res = SysNotice::from(data);
    let data = CONTEXT.sys_notice_service.add(&res).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/notice")]
#[has_permit("system:notice:edit")]
pub async fn update(arg: web::Json<NoticeUpdateDTO>) -> impl Responder {
    let mut data = SysNotice::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_notice_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/notice/{notice_id}")]
#[has_permit("system:notice:remove")]
pub async fn remove(notice_id: web::Path<String>) -> impl Responder {
    let notice_id = notice_id.into_inner();
    let data = CONTEXT
        .sys_notice_service
        .remove(&notice_id)
        .await;
    RespVO::<u64>::judge(data.unwrap(), "".to_string(), "删除失败！".to_string()).resp_json()
}
