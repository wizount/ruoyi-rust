use crate::domain::dto::{DeptQueryDTO};
use crate::domain::table::SysDept;

use crate::domain::vo::{DeptTreeVO, SysDeptVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

//const DICT_KEY: &'static str = "sys_dept:all";

/// dictionary service
pub struct SysDeptService {}

impl SysDeptService {
    pub async fn all(&self, arg: &DeptQueryDTO) -> Result<Vec<SysDeptVO>> {
        let data = SysDept::select_all_query(pool!(), arg).await?;
        let mut res = vec![];
        for d in data {
            res.push(SysDeptVO::from(d));
        }
        Ok(res)
    }


    pub async fn detail(&self, dept_id: &str) -> Result<SysDeptVO> {
        let dept = SysDept::select_by_column(pool!(), field_name!(SysDept.dept_id), dept_id)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dept_id)))?;
        let dept_vo = SysDeptVO::from(dept);
        return Ok(dept_vo);
    }

    pub async fn add(&self, arg: &SysDept) -> Result<u64> {
        let old = SysDept::select_by_column(
            pool!(),
            rbatis::field_name!(SysDept.dept_id),
            arg.dept_id.as_deref().unwrap_or_default(),
        )
            .await?;
        // if old.len() > 0 {
        //     return Err(Error::from(format!(
        //         "字典已存在! code={}",
        //         arg.code.as_deref().unwrap_or_default()
        //     )));
        // }
        let result = Ok(SysDept::insert(pool!(), &arg).await?.rows_affected);
        return result;
    }

    pub async fn update(&self, data: SysDept) -> Result<u64> {
        let result = SysDept::update_by_column(pool!(), &data, "dept_id").await;
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, dept_id: &str) -> Result<u64> {
        let targets = SysDept::select_by_column(pool!(), "dept_id", dept_id).await?;

        let r = SysDept::delete_by_column(pool!(), "dept_id", dept_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dept", &targets).await?;
        }
        Ok(r.rows_affected)
    }
    //根据user id获得本单位及下属单位部门列表
    pub async fn get_dept_tree(&self, user_id: &str) -> Result<Vec<DeptTreeVO>> {
        let depts = SysDept::select_all_query(pool!(), &DeptQueryDTO {// todo
            dept_name: None,
            status: None,
        }).await?;
        let mut res = vec![];
        for d in depts {
            res.push(DeptTreeVO::from(d));
        }
       self. build_dept_tree(&res)
    }
    ///An depts array with a hierarchy
    pub fn build_dept_tree(&self, all_depts: &Vec<DeptTreeVO>) -> Result<Vec<DeptTreeVO>> {
        //find tops
        let mut tops = vec![];
        for item in all_depts {
            //parent id null, it is an top menu
            if item.is_parent() {
                tops.push(item.clone());
            }
        }
        //find child
      //  tops.sort_by(|a, b| a.order_num.cmp(&b.order_num));
        for mut item in &mut tops {
            self.loop_find_children(&mut item, &all_depts);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_children(&self, arg: &mut DeptTreeVO, all_depts: &Vec<DeptTreeVO>) {
        let mut children = vec![];
        for item in all_depts {
            if !item.is_parent() && item.parent_id == arg.id {
                let mut item = item.clone();
                self.loop_find_children(&mut item, all_depts);
                children.push(item);
            }
        }
        if !children.is_empty() {
         //   children.sort_by(|a, b| a.order_num.cmp(&b.order_num));
            arg.children = Some(children);
        }
    }

}
