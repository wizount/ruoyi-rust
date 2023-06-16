use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogininforPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub state: Option<i32>,
}

impl From<LogininforPageDTO> for PageRequest {
    fn from(arg: LogininforPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&LogininforPageDTO> for PageRequest {
    fn from(arg: &LogininforPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

