use merge_yaml_hash::MergeYamlHash;
use std::collections::HashMap;
use std::fs;

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub base_api: String,
    pub db_url: String,
    pub db_pool_len: usize,
    pub db_pool_timeout: usize,
    pub log_dir: String,
    pub log_rolling: String,
    pub log_pack_compress: String,
    pub log_keep_type: String,
    pub log_level: String,
    pub log_chan_len: Option<usize>,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub jwt_exp: usize,
    pub jwt_refresh_token: usize,

    pub captcha_expired_min: u64,
    pub token_expired_min: u64,
    pub chn_pwd_check: bool,
    pub address_enabled: bool,
    pub apihz_id: String,
    pub apihz_key: String,
    pub cache: String,
    pub storage: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub trash_recycle_days: u64,
    pub datetime_format: String,
    pub errors: HashMap<String, String>,
    pub error_infos: Option<HashMap<String, String>>,

    pub upload_path: String,
    pub upload_max_size: usize,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yml");

        let mut hash = MergeYamlHash::new();

        hash.merge(yml_data);

        //自动识别，加载不同配置
        #[cfg(debug_assertions)]
        let dev_or_release_yml = "application-dev.yml";
        #[cfg(not(debug_assertions))]
        let dev_or_release_yml = "application-prod.yml";

        match fs::read_to_string(dev_or_release_yml) {
            Ok(s) => {
                println!("loading profile {}", dev_or_release_yml);
                hash.merge(&s);
            }
            Err(_) => {
                panic!("Can't load {}", dev_or_release_yml);
            }
        }

        //load config
        let result: ApplicationConfig = serde_yaml::from_str(&*hash.to_string()).expect("load config file fail");
        if result.debug {
            println!("[ruoyi_rust] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[ruoyi_rust] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}

impl ApplicationConfig {
    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => match self.errors.get("-1") {
                None => "unknown error".to_string(),
                Some(v) => v.to_string(),
            },
            Some(v) => v.as_str().to_string(),
        }
    }

    pub fn init_infos(&mut self) {
        self.error_infos = Some(HashMap::new());
        for (k, error) in &self.errors {
            let mut error = error.to_string();
            if error.contains(",") {
                error = error[0..error.find(",").unwrap()].to_string();
            }
            self.error_infos.as_mut().unwrap().insert(error, k.to_string());
        }
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::context::CONTEXT.config.get_error_info($code)
    };
}
