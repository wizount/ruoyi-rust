use ruoyi_rust::service::CONTEXT;
use ruoyi_rust::http_server;


/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    ruoyi_rust::config::log::init_log();


    //连接数据库
    CONTEXT.init_pool().await;

    CONTEXT.sys_dict_data_service.update_cache().await?;
    CONTEXT.sys_config_service.loading_config_cache().await?;
    CONTEXT.sys_menu_service.update_cache().await?;

    http_server::build_server(&CONTEXT.config.base_api).await
}
