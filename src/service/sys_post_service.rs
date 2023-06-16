use rbatis::sql::{Page, PageRequest};

use crate::domain::dto::PostPageDTO;
use crate::domain::table::SysPost;
use crate::domain::vo::SysPostVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

//const DICT_KEY: &'static str = "sys_post:all";

/// dictionary service
pub struct SysPostService {}

impl SysPostService {
    pub async fn page(&self, arg: &PostPageDTO) -> Result<Page<SysPostVO>> {
        let data = SysPost::select_page(
            pool!(),
            &PageRequest::from(arg),
            arg
        )
            .await?;
        let page = Page::<SysPostVO>::from(data);

        Ok(page)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysPostVO>> {
        let data = SysPost::select_all(pool!()).await?;
        let mut post_vos = vec![];
        for s in data {
            post_vos.push(SysPostVO::from(s));
        }
        Ok(post_vos)
    }
    pub async fn detail(&self, post_id: &str) -> Result<SysPostVO> {
        let post = SysPost::select_by_column(pool!(), field_name!(SysPost.post_id), post_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", post_id)))?;
        let post_vo = SysPostVO::from(post);
        return Ok(post_vo);
    }

    pub async fn add(&self, arg: &SysPost) -> Result<u64> {
        let old = SysPost::select_by_column(
            pool!(),
            rbatis::field_name!(SysPost.post_id),
            arg.post_id.as_deref().unwrap_or_default(),
        )
            .await?;
        let result = Ok(SysPost::insert(pool!(), &arg).await?.rows_affected);
        return result;
    }

    pub async fn update(&self, data: SysPost) -> Result<u64> {
        let result = SysPost::update_by_column(pool!(), &data, "post_id").await;
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, post_id: &str) -> Result<u64> {
        let targets = SysPost::select_by_column(pool!(), "post_id", post_id).await?;

        let r = SysPost::delete_by_column(pool!(), "post_id", post_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_post", &targets).await?;
        }
        Ok(r.rows_affected)
    }
}
