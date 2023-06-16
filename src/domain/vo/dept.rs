use rbatis::rbdc::datetime::DateTime;
use crate::domain::table::{SysDept};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDeptVO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
}

impl From<SysDept> for SysDeptVO {
    fn from(arg: SysDept) -> Self {
        Self {
            dept_id: arg.dept_id,
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: None,
        }
    }
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptTreeVO {
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub parent_id: Option<String>,
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptTreeVO>>,
}

impl From<SysDept> for DeptTreeVO {
    fn from(arg: SysDept) -> Self {
        Self {
            id: arg.dept_id,
            parent_id: arg.parent_id,
            label: arg.dept_name,
            children: None,
        }
    }
}
impl DeptTreeVO {
    pub fn is_parent(&self) -> bool {
        self.parent_id.is_none() || self.parent_id.clone().unwrap_or_default().eq("0")
    }
}
