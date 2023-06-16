use actix_http::header::{HeaderMap, USER_AGENT};
use actix_web::{HttpRequest};
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use user_agent_parser::UserAgentParser;

use crate::domain::table::SysLogininfor;

// //将actix 上传保存在本地磁盘
// pub async fn save_http_files(mut payload: Multipart) -> Result<Vec<(String, String)>, Error> {
//     // iterate over multipart stream
//     let mut files = vec![];
//     while let Some(mut field) = payload.try_next().await? {
//         // A multipart/form-data stream has to contain `content_disposition`
//         let content_disposition = field.content_disposition();
//         let filename = content_disposition
//             .get_filename().unwrap().to_string();
//         let filepath = format!("./tmp/{}", Uuid::new_v4().to_string());
//         let filepath_ = filepath.clone();
//         // File::create is blocking operation, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath)).await??;
//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.try_next().await? {
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
//         }
//         files.push((filepath_, filename));
//     }
//     Ok(files)
// }

fn get_header_value(headers: &HeaderMap, head: &str) -> String {
    if headers.get(head).is_some() {
        headers.get(head).unwrap().to_str().unwrap_or("").to_string()
    } else {
        "".to_string()
    }
}

pub fn get_ip_addr(req: &HttpRequest) -> String
{
    let headers = req.headers();
    let mut ip = get_header_value(headers, "x-forwarded-for");
    if ip.len() == 0 { ip = get_header_value(headers, "Proxy-Client-IP") }
    if ip.len() == 0 { ip = get_header_value(headers, "X-Forwarded-For") }
    if ip.len() == 0 { ip = get_header_value(headers, "WL-Proxy-Client-IP") }
    if ip.len() == 0 { ip = get_header_value(headers, "X-Real-IP") }
    if ip.len() == 0 {
        return req.peer_addr().unwrap().ip().to_string();
    }
    ip.to_string()
    //return "0:0:0:0:0:0:0:1".equals(ip)?;
    // "127.0.0.1": getMultistageReverseProxyIp(ip);
}

pub fn build_logininfor(req: &HttpRequest,username:String,status:char,msg:String) ->SysLogininfor{
    let ua_parser = UserAgentParser::from_path("./user_agent.yml").unwrap();
    let user_agent = req.headers().get(USER_AGENT).unwrap().to_str().unwrap_or("");
    let os = ua_parser.parse_os(user_agent).name.unwrap().to_string();
    SysLogininfor {
        info_id: ObjectId::new().to_string().into(),
        user_name: Some(username),
        ipaddr: Some(crate::util::web_utils::get_ip_addr(&req)),
        login_location: None,
        browser: None,
        os: Some(os),
        status: Some(status),
        msg: Some(msg),
        login_time:  DateTime::now().set_micro(0).into(),
    }
}

// pub(crate) fn timestamp() -> i64 {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
//     ms
// }