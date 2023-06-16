use rbatis::object_id::ObjectId;
use crate::domain::table::SysPost;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

/// page page DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub post_name: Option<String>,
    pub post_code: Option<String>,
    pub status: Option<char>,
}

impl From<PostPageDTO> for PageRequest {
    fn from(arg: PostPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&PostPageDTO> for PageRequest {
    fn from(arg: &PostPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAddDTO {
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<PostAddDTO> for SysPost {
    fn from(arg: PostAddDTO) -> Self {
        SysPost {
            post_id: ObjectId::new().to_string().into(),
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
            status: arg.status,
            create_by: None,
            create_time: DateTime::now().set_micro(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostUpdateDTO {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<PostUpdateDTO> for SysPost {
    fn from(arg: PostUpdateDTO) -> Self {
        SysPost {
            post_id: arg.post_id,
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_micro(0).into(),
            remark: arg.remark,
        }
    }
}
