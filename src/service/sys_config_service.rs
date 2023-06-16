use rbatis::sql::{Page, PageRequest};
use rbs::to_value;
use crate::domain::dto::ConfigPageDTO;
use crate::domain::table::SysConfig;
use crate::domain::vo::SysConfigVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

const SYS_CONFIG_KEY: &'static str = "sys_config:";

/// dictionary service
pub struct SysConfigService {}

impl SysConfigService {
    pub async fn page(&self, arg: &ConfigPageDTO) -> Result<Page<SysConfigVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysConfig::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysConfigVO>::from(data);
        Ok(page)
    }


    pub async fn detail(&self, config_id: &str) -> Result<SysConfigVO> {
        let config = SysConfig::select_by_column(pool!(), field_name!(SysConfig.config_id), config_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", config_id)))?;
        let config_vo = SysConfigVO::from(config);
        return Ok(config_vo);
    }

    pub async fn add(&self, config: &SysConfig) -> Result<u64> {
        if !self.check_config_key_unique("", config.config_key.as_ref().unwrap()).await? { return Err(Error::from("参数键名重复！")); }

        let result = SysConfig::insert(pool!(), &config).await?.rows_affected;
        if result == 1 {
            self.add_to_cache(config).await;
        }
        Ok(result)
    }

    pub async fn update(&self, config: SysConfig) -> Result<u64> {
        let old_config = SysConfig::select_by_column(pool!(), "config_id", config.config_id.as_ref().unwrap()).await?.into_iter().next();
        match old_config {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => {
                if !c.config_key.clone().unwrap().eq(config.config_key.as_ref().unwrap()) {
                    CONTEXT.cache_service.del(&self.get_cache_key(&c.config_key.unwrap())).await;
                }
            }
        }
        if !self.check_config_key_unique(config.config_id.as_ref().unwrap(), config.config_key.as_ref().unwrap()).await? { return Err(Error::from("参数键名重复！")); }
        let result = SysConfig::update_by_column(pool!(), &config, "config_id").await?.rows_affected;
        if result == 1 {
            self.add_to_cache(&config).await;
        }
        Ok(result)
    }

    pub async fn remove(&self, config_id: &str) -> Result<u64> {
        let targets = SysConfig::select_by_column(pool!(), "config_id", config_id).await?;
        let r = SysConfig::delete_by_column(pool!(), "config_id", config_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_config", &targets).await;
            CONTEXT.cache_service.del(&self.get_cache_key(targets.into_iter().next().unwrap().config_key.clone().as_ref().unwrap())).await;
        }
        Ok(r.rows_affected)
    }


    /**
     * 获取验证码开关，默认不打开
     *
     * @return true开启，false关闭
     */
    pub async fn select_captcha_enabled(&self) -> Result<bool>
    {
        let captcha_enabled = self.select_config_by_key("sys.account.captcha_enabled").await;
        match captcha_enabled {
            Ok(s) => {
                Ok(s.eq("true"))
            }
            Err(_) => { Ok(false) }
        }
    }

    /**
     * 根据键名查询参数配置信息
     *
     */
    pub async fn select_config_by_key(&self, config_key: &str) -> Result<String>
    {
        let config_value = CONTEXT.cache_service.get_string(&self.get_cache_key(config_key)).await?;
        if !config_value.is_empty() {
            return Ok(config_value);
        }
        let config = SysConfig::select_by_column(pool!(), "config_key", config_key).await?;
        match config.into_iter().next() {
            None => { Ok("".to_string()) }
            Some(c) => {
                CONTEXT.cache_service.set_string(&self.get_cache_key(c.config_key.as_ref().unwrap()), c.config_value.clone().as_ref().unwrap()).await;
                Ok(c.config_value.unwrap())
            }
        }
    }
    /**
     * 校验参数键名是否唯一
     *
     */
    pub async fn check_config_key_unique(&self, config_id: &str, config_key: &str) -> Result<bool>
    {
        let count: u64 = pool!()
            .query_decode("select count(1) as count from sys_config where config_id!=? and config_key =?", vec![to_value!(config_id), to_value!(config_key)])
            .await
            .unwrap();
        Ok(count < 1)
    }
    /*
     清空参数缓存数据
    */
    pub async fn clear_config_cache(&self) -> Result<()>
    {
        let keys = CONTEXT.cache_service.keys("sys_config:*").await?;
        for k in keys {
            CONTEXT.cache_service.del(&k).await;
        }
        Ok(())
    }

    /**
     * 重置参数缓存数据
     */

    pub async fn reset_config_cache(&self) -> Result<()>
    {
        self.clear_config_cache();
        self.loading_config_cache();
        Ok(())
    }
    //加载所有的到redis
    pub async fn loading_config_cache(&self) -> Result<u64> {
        let config_list = SysConfig::select_all(pool!()).await?;
        for config in config_list {
            self.add_to_cache(&config).await;
        }
        Ok(1)
    }
    async fn add_to_cache(&self, config: &SysConfig) {
        CONTEXT.cache_service.set_string(&self.get_cache_key(config.config_key.as_ref().unwrap()), config.config_value.as_ref().unwrap()).await;
    }
    //对config_key进行处理
    fn get_cache_key(&self, origin: &str) -> String {
        format!("{}{}", SYS_CONFIG_KEY, origin)
    }
}
