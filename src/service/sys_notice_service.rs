use rbatis::sql::{Page, PageRequest};

use crate::domain::dto::NoticePageDTO;
use crate::domain::table::SysNotice;
use crate::domain::vo::SysNoticeVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

/// notice service
pub struct SysNoticeService {}

impl SysNoticeService {
    pub async fn page(&self, arg: &NoticePageDTO) -> Result<Page<SysNoticeVO>> {
        let data = SysNotice::select_page(
            pool!(),
            &PageRequest::from(arg),
            arg
        )
            .await?;
        let page = Page::<SysNoticeVO>::from(data);

        Ok(page)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysNoticeVO>> {
        let data = SysNotice::select_all(pool!()).await?;
        let mut notice_vos = vec![];
        for s in data {
            notice_vos.push(SysNoticeVO::from(s));
        }
        Ok(notice_vos)
    }
    pub async fn detail(&self, notice_id: &str) -> Result<SysNoticeVO> {
        let notice = SysNotice::select_by_column(pool!(), field_name!(SysNotice.notice_id), notice_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", notice_id)))?;
        let notice_vo = SysNoticeVO::from(notice);
        return Ok(notice_vo);
    }

    pub async fn add(&self, arg: &SysNotice) -> Result<u64> {
        let old = SysNotice::select_by_column(
            pool!(),
            rbatis::field_name!(SysNotice.notice_id),
            arg.notice_id.as_deref().unwrap_or_default(),
        )
            .await?;
        let result = Ok(SysNotice::insert(pool!(), &arg).await?.rows_affected);
        return result;
    }

    pub async fn update(&self, data: SysNotice) -> Result<u64> {
        let result = SysNotice::update_by_column(pool!(), &data, "notice_id").await;
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, notice_id: &str) -> Result<u64> {
        let targets = SysNotice::select_by_column(pool!(), "notice_id", notice_id).await?;

        let r = SysNotice::delete_by_column(pool!(), "notice_id", notice_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_notice", &targets).await?;
        }
        Ok(r.rows_affected)
    }
}
