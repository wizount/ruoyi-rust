use rbatis::sql::{Page, PageRequest};
use rbs::to_value;

use crate::domain::dto::{DictTypePageDTO};
use crate::domain::table::{ SysDictType};
use crate::domain::vo::SysDictTypeVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

//const DICT_KEY: &'static str = "sys_dict_type:all";

/// dictionary service
pub struct SysDictTypeService {}

impl SysDictTypeService {
    pub async fn page(&self, arg: &DictTypePageDTO) -> Result<Page<SysDictTypeVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysDictType::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictTypeVO>::from(data);
        Ok(page)
    }
    pub async fn finds_all(&self) -> Result<Vec<SysDictTypeVO>> {
        let data = SysDictType::select_all(pool!()).await?;
        let mut dict_type_vos = vec![];
        for d in data {
            dict_type_vos.push(SysDictTypeVO::from(d));
        }
        Ok(dict_type_vos)
    }
    pub async fn detail(&self, dict_id: &str) -> Result<SysDictTypeVO> {
        let dict_type = SysDictType::select_by_column(pool!(), field_name!(SysDictType.dict_id), dict_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_id)))?;
        let dict_type_vo = SysDictTypeVO::from(dict_type);
        return Ok(dict_type_vo);
    }
    pub async fn add(&self, arg: &SysDictType) -> Result<u64> {
        let old = SysDictType::select_by_column(
            pool!(),
            rbatis::field_name!(SysDictType.dict_id),
            arg.dict_id.as_deref().unwrap_or_default(),
        )
            .await?;
        // if old.len() > 0 {
        //     return Err(Error::from(format!(
        //         "字典已存在! code={}",
        //         arg.code.as_deref().unwrap_or_default()
        //     )));
        // }
        let result = Ok(SysDictType::insert(pool!(), &arg).await?.rows_affected);
        CONTEXT.sys_dict_data_service.update_cache().await;
        return result;
    }

    pub async fn update(&self, data: SysDictType) -> Result<u64> {
        let result = SysDictType::update_by_column(pool!(), &data, "dict_id").await;
        if result.is_ok() {
            //更新dict_data
            CONTEXT.sys_dict_data_service.update_cache().await?;
        }
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, dict_id: &str) -> Result<u64> {
        let targets = SysDictType::select_by_column(pool!(), "dict_id", dict_id).await?;
        if targets.len() == 1 {
            let dict_type=targets.get(0).unwrap().dict_type.clone().unwrap();
            let count: u64 = pool!()
                .query_decode("select count(1) as count from sys_dict_type where dict_type =?", vec![to_value!(dict_type)])
                .await
                .unwrap();
            if count > 0 { return Err(Error::from("存在子项,不允许删除！")); }

        } else {  return Err(Error::from(format!("字典id{}不存在！", dict_id)));}

        let r = SysDictType::delete_by_column(pool!(), "dict_id", dict_id).await?;
        if r.rows_affected > 0 {
            CONTEXT.sys_dict_data_service.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_type", &targets).await?;
        }
        Ok(r.rows_affected)
    }
}
