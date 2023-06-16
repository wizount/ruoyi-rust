use actix_easy_multipart::MultipartFormConfig;
use crate::controller::{img_controller, sys_auth_controller, sys_config_controller, sys_dept_controller, sys_dict_data_controller, sys_dict_type_controller, sys_menu_controller, sys_notice_controller, sys_post_controller, sys_profile_controller, sys_role_controller, sys_user_controller};
use crate::service::CONTEXT;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web_validator::{Error, JsonConfig};
use actix_web_validator::error::flatten_errors;
use crate::controller::monitor::{server_controller, sys_logininfor_controller, sys_user_online_controller};
use crate::domain::vo::RespVO;

// #[get("/")]
// async fn index() -> impl Responder {
//     NamedFile::open_async("../dist/index.html").await.unwrap()
// }
//
// #[get("/")]
// async fn fav() -> impl Responder {
//     NamedFile::open_async("../dist/f").await.unwrap()
// }

pub fn build_server(base_api: &'static str) -> Server {
    HttpServer::new(|| {
        //定义validate 错误的返回json
        //目前只支持Json，form query path需要再写，目前用不上
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            match err {
                Error::Validate(e) => {
                    let err_str = flatten_errors(&e).iter()
                        .map(|(_, field, err)| { format!("{}", err) })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let resp: RespVO<String> = RespVO {
                        code: 500,
                        msg: Some(err_str),
                        data: None,
                    };
                    actix_web::error::InternalError::from_response(e, resp.resp_json()).into()
                }
                _ => {
                    actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
                }
            }
        });
        let base_api = base_api.to_string();
        App::new()
            //  .wrap(Auth {})
            .app_data(json_config)
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
            .app_data(
                MultipartFormConfig::default()
                    .memory_limit(50 * 1024 * 1024)
                    .total_limit(50 * 1024 * 1024),
            )
           // .service(index)
            //.service(fs::Files::new("/assets", "../dist/assets").show_files_listing())
            .service(web::scope(&base_api).service(
                web::scope("/system") //系统应用
                    .service(sys_menu_controller::role_menu_treeselect)
                    .service(sys_menu_controller::treeselect)
                    .service(sys_menu_controller::query_menu)
                    .service(sys_menu_controller::detail)
                    .service(sys_menu_controller::add)
                    .service(sys_menu_controller::update)
                    .service(sys_menu_controller::remove)

                    .service(sys_profile_controller::profile)
                    .service(sys_profile_controller::update_profile)
                    .service(sys_profile_controller::update_pwd)

                    .service(sys_user_controller::page)
                    .service(sys_user_controller::get_dept_tree)
                    .service(sys_user_controller::detail)
                    .service(sys_user_controller::add)
                    .service(sys_user_controller::update)
                    .service(sys_user_controller::remove)
                    .service(sys_user_controller::set_auth_roles)
                    .service(sys_user_controller::before_add)
                    .service(sys_user_controller::get_auth_roles)
                    .service(sys_user_controller::change_status)


                    .service(sys_role_controller::page)
                    .service(sys_role_controller::detail)
                    .service(sys_role_controller::add)
                    .service(sys_role_controller::update)
                    .service(sys_role_controller::remove)
                    .service(sys_role_controller::auth_user_list)
                    .service(sys_role_controller::unallocated_user_list)
                    .service(sys_role_controller::change_status)
                    .service(sys_role_controller::cancel_user)
                    .service(sys_role_controller::auth_user_all)
                    .service(sys_role_controller::cancel_user_all)


                    .service(sys_dict_type_controller::optionselect)
                    .service(sys_dict_type_controller::page)
                    .service(sys_dict_type_controller::detail)
                    .service(sys_dict_type_controller::add)
                    .service(sys_dict_type_controller::update)
                    .service(sys_dict_type_controller::remove)

                    .service(sys_dict_data_controller::page)
                    .service(sys_dict_data_controller::detail)
                    .service(sys_dict_data_controller::add)
                    .service(sys_dict_data_controller::update)
                    .service(sys_dict_data_controller::remove)
                    .service(sys_dict_data_controller::get_by_dict_type)

                    .service(sys_config_controller::page)
                    .service(sys_config_controller::detail)
                    .service(sys_config_controller::add)
                    .service(sys_config_controller::update)
                    .service(sys_config_controller::remove)

                    .service(sys_dept_controller::page)
                    .service(sys_dept_controller::detail)
                    .service(sys_dept_controller::add)
                    .service(sys_dept_controller::update)
                    .service(sys_dept_controller::remove)

                    .service(sys_post_controller::page)
                    .service(sys_post_controller::detail)
                    .service(sys_post_controller::add)
                    .service(sys_post_controller::update)
                    .service(sys_post_controller::remove)

                    .service(sys_notice_controller::page)
                    .service(sys_notice_controller::detail)
                    .service(sys_notice_controller::add)
                    .service(sys_notice_controller::update)
                    .service(sys_notice_controller::remove)
            ).service(
                web::scope("/monitor") //系统应
                    .service(sys_logininfor_controller::page)
                    .service(sys_logininfor_controller::remove)


                    .service(sys_user_online_controller::page)
                    .service(sys_user_online_controller::force_logout)

                    .service(server_controller::server_info)
            ).route("/captchaImage", web::get().to(img_controller::captcha))
                .route(
                    "/login",
                    web::post().to(sys_auth_controller::login),
                )
                .route(
                    "/getInfo",
                    web::get().to(sys_auth_controller::info),
                )
                .route(
                    "/getRouters",
                    web::get().to(sys_menu_controller::routers),
                )
                .route(
                    "/logout",
                    web::post().to(sys_auth_controller::logout),
                )
            )
    })
        .bind(&CONTEXT.config.server_url).unwrap()
        .run()
}
