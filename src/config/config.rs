use std::fs;
use merge_yaml_hash::{MergeYamlHash};

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub base_api: String,
    pub web_redis_url: String,
    pub database_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_chan_len: Option<usize>,
    pub log_level: String,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub white_list_api: Vec<String>,
    pub cache_type: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub captcha_enabled: bool,
    pub captcha_expired_min: u64,
    pub token_expired_min: u64
}

impl Default for ApplicationConfig {
    fn default() -> Self {

        let k = "profile=";
        let profile = std::env::args_os()
            .find(|v| v.to_str().unwrap_or_default().starts_with(k))
            .map(|v| v.to_str().unwrap_or_default().trim_start_matches(k).to_string());
        let profile = match profile{
            Some(o) => Some(o),
            None => { 
                std::env::var("profile").ok()
             }
        };
        let profile = match profile{
            Some(o) => Some(o),
            None => { 
                std::env::var_os("profile").map(|x|x.into_string().unwrap_or_default())
             }
        };
        let profile = profile.unwrap_or_else(|| "prod".to_string());
        let profile = profile.trim().to_string();
        println!("loading profile: {}", profile);
        let yml_data = include_str!("../../application.yml");

        let mut hash = MergeYamlHash::new();

        hash.merge(yml_data);
        if profile.len() > 0 {
            match fs::read_to_string(format!("application-{profile}.yml")) {
                Ok(s) => { hash.merge(&s); }
                Err(_) => { println!("Can't loading application-{profile}.yml"); }
            }
        }
        //load config
        let result: ApplicationConfig =
            serde_yaml::from_str(&*hash.to_string()).expect("load config file fail");
        if result.debug {
            //println!("[ruoyi_rust] load config:{:?}", result);
            println!("[ruoyi_rust] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[ruoyi_rust] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
