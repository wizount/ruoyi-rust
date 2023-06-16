use rbatis::sql::{Page, PageRequest};

use crate::domain::dto::LogininforPageDTO;
use crate::domain::table::SysLogininfor;
use crate::domain::vo::SysLogininforVO;
use crate::error::Result;
use crate::pool;


/// dictionary service
pub struct SysLogininforService {}

impl SysLogininforService {
    pub async fn page(&self, arg: &LogininforPageDTO) -> Result<Page<SysLogininforVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysLogininfor::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysLogininforVO>::from(data);
        Ok(page)
    }

    //异步加入日志
    pub async fn add_async(&self, arg: &SysLogininfor) -> Result<u64> {
        let info=arg.to_owned();
        tokio::spawn(async move {
            SysLogininfor::insert(pool!(), &info).await;
        });
        return Ok(1);
    }


    pub async fn remove(&self, info_id: &str) -> Result<u64> {
        let r = SysLogininfor::delete_by_column(pool!(), "info_id", info_id).await?;
        Ok(r.rows_affected)
    }
}
