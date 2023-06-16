use actix_web::{ get, Responder, HttpRequest};

use crate::domain::vo::{RespVO};
use permit_lib::has_permit;
use crate::util::hardware::get_server_info;

#[get("/server")]
#[has_permit("monitor:server:list")]
pub async fn server_info() -> impl Responder {
    RespVO::from_result(&Ok(get_server_info())).resp_json()
}


