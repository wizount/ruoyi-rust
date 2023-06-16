use crate::domain::dto::{PostAddDTO, PostUpdateDTO, PostPageDTO};
use crate::domain::table::SysPost;
use crate::domain::vo::{PageVO, RespVO};
use crate::service::CONTEXT;
use actix_web::{get, post, put, delete, web,HttpRequest, Responder};
use crate::config::global_variables::STATUS_NORMAL;
use permit_lib::has_permit;

#[get("/post/list")]
#[has_permit("system:post:query")]
pub async fn page(query: web::Query<PostPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_post_service.page(&query).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/post/{dict_id}")]
#[has_permit("system:post:query")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let post_vo = CONTEXT.sys_post_service.detail(&dict_id).await;
    RespVO::from_result(&post_vo).resp_json()
}


#[post("/post")]
#[has_permit("system:post:add")]
pub async fn add(arg: web::Json<PostAddDTO>) -> impl Responder {
    let mut data = SysPost::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_post_service.add(&data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "添加失败！".to_string()).resp_json()
}

#[put("/post")]
#[has_permit("system:post:edit")]
pub async fn update(arg: web::Json<PostUpdateDTO>) -> impl Responder {
    let mut data = SysPost::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_post_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}

#[delete("/post/{post_id}")]
#[has_permit("system:post:remove")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT
        .sys_post_service
        .remove(&dict_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
