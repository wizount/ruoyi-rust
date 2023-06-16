use std::collections::HashMap;
use rbatis::sql::{Page, PageRequest};

use crate::domain::dto::{ DictDataPageDTO};
use crate::domain::table::SysDictData;
use crate::domain::vo::{SysDictDataSimpleVO, SysDictDataVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

//const DICT_KEY: &'static str = "sys_dict_data:all";

/// dictionary service
pub struct SysDictDataService {}

impl SysDictDataService {
    pub async fn page(&self, arg: &DictDataPageDTO) -> Result<Page<SysDictDataVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysDictData::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictDataVO>::from(data);
        Ok(page)
    }

    pub async fn get_by_dict_type(&self, dict_type: &String) -> Result<Vec<SysDictDataSimpleVO>> {
        let data = SysDictData::select_by_dict_type(pool!(), &dict_type).await?;
        let mut res = vec![];
        for d in data {
            res.push(SysDictDataSimpleVO::from(d))
        }
        Ok(res)
    }

    pub async fn detail(&self, dict_code: &str) -> Result<SysDictDataVO> {
        let dict_data = SysDictData::select_by_column(pool!(), field_name!(SysDictData.dict_code), dict_code)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_code)))?;
        let dict_data_vo = SysDictDataVO::from(dict_data);
        return Ok(dict_data_vo);
    }

    pub async fn add(&self, arg: &SysDictData) -> Result<u64> {
        let old = SysDictData::select_by_column(
            pool!(),
            rbatis::field_name!(SysDictData.dict_code),
            arg.dict_code.as_deref().unwrap_or_default(),
        )
            .await?;
        // if old.len() > 0 {
        //     return Err(Error::from(format!(
        //         "字典已存在! code={}",
        //         arg.code.as_deref().unwrap_or_default()
        //     )));
        // }
        let result = Ok(SysDictData::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    pub async fn update(&self, data: SysDictData) -> Result<u64> {
        let result = SysDictData::update_by_column(pool!(), &data, "dict_code").await;
        if result.is_ok() {
            self.update_cache().await?;
        }
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, dict_code: &str) -> Result<u64> {
        let targets = SysDictData::select_by_column(pool!(), "dict_code", dict_code).await?;

        let r = SysDictData::delete_by_column(pool!(), "dict_code", dict_code).await?;
        if r.rows_affected > 0 {
            self.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_data", &targets).await?;
        }
        Ok(r.rows_affected)
    }

    /// update for all cache
    pub async fn update_cache(&self) -> Result<()> {
        let mut all = SysDictData::select_all(pool!()).await?;
        all.sort_by(|a, b| a.dict_sort.cmp(&b.dict_sort));
        let mut dict_data_map: HashMap<&str, Vec<SysDictDataSimpleVO>> = HashMap::new();
        for dict_data in all {
            let key = dict_data.dict_type.clone().as_deref().unwrap();
            let key = "1233";//fixme
            let data_sim = SysDictDataSimpleVO::from(dict_data);
            if dict_data_map.contains_key(key) {
                dict_data_map.get_mut(key).unwrap().push(data_sim);
            } else {
                dict_data_map.insert(key, vec![data_sim]);
            }
        }
        //  CONTEXT.cache_service.set_json(DICT_KEY, &dict_data_map).await?;
        return Ok(());
    }
}
