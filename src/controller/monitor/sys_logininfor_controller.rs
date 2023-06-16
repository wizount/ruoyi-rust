use actix_web::{delete, get,   Responder, web,HttpRequest};

use crate::domain::dto::LogininforPageDTO;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use permit_lib::has_permit;

#[get("/logininfor/list")]
#[has_permit("monitor:logininfor:query")]
pub async fn page(page: web::Query<LogininforPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_logininfor_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[delete("/logininfor/{info_id}")]
#[has_permit("monitor:logininfor:remove")]
pub async fn remove(info_id: web::Path<String>) -> impl Responder {
    let info_id = info_id.into_inner();
    let data = CONTEXT
        .sys_logininfor_service
        .remove(&info_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
