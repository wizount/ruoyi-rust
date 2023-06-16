pub mod dict_type;
pub mod dict_data;
pub mod jwt;
pub mod menu;
pub mod router;
pub mod role;
pub mod sign_in;
pub mod user;
pub mod config;
pub mod sys_logininfor;
pub mod dept;
pub mod post;
pub mod monitor;
pub mod notice;

pub use dict_type::*;
pub use dict_data::*;
pub use jwt::*;
pub use menu::*;
pub use router::*;
pub use role::*;
pub use sign_in::*;
pub use user::*;
pub use config::*;
pub use dept::*;
pub use sys_logininfor::*;
pub use post::*;
pub use monitor::*;
pub use notice::*;

use crate::error::Error;
use actix_web::HttpResponse;
use rbatis::sql::Page;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CODE_SUCCESS: u64 = 200;
pub const CODE_FAIL: u64 = 500;

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RespVO<T> {
    pub code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "camelCase")]
    pub data: Option<T>,
}

impl<T> RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: CODE_SUCCESS,
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: CODE_FAIL,
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: CODE_SUCCESS,
            msg: None,
            data: Some(arg.clone()),
        }
    }
    pub fn from_success_info(msg: &str) -> Self {
        Self {
            code: CODE_SUCCESS,
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn from_error_result(code: u64, arg: &Result<T, Error>) -> Self {
        Self {
            code,
            msg: Some(arg.clone().err().unwrap().to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: u64, info: &str) -> Self {
        Self {
            code,
            msg: Some(info.to_string()),
            data: None,
        }
    }


    pub fn judge(affected: u64, success_msg: String, fail_message: String) -> Self {
        if affected >= 1 {
            Self {
                code: CODE_SUCCESS,
                msg: Some(success_msg),
                data: None,
            }
        } else {
            Self {
                code: CODE_FAIL,
                msg: Some(fail_message),
                data: None,
            }
        }
    }
    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

impl<T> ToString for RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// 自定义输入，serde_json map
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespJson {
    inner: serde_json::map::Map<String, Value>,
}

impl RespJson
{

    pub fn new() -> Self {
        Self {
            inner: serde_json::map::Map::new()
        }
    }
    pub fn success() -> Self {
        let mut inner = serde_json::map::Map::new();
        inner.insert("code".to_string(), CODE_SUCCESS.into());
        Self {
            inner
        }
    }
    pub fn success_info(msg:&str) -> Self {
        let mut inner = serde_json::map::Map::new();
        inner.insert("code".to_string(), CODE_SUCCESS.into());
        inner.insert("msg".to_string(), msg.into());
        Self {
            inner
        }
    }
    //插入新的
    pub fn insert(&mut self, key: String, v: Value) -> &mut RespJson {
        self.inner.insert(key, v);
        self
    }
    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

impl ToString for RespJson
{
    fn to_string(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageVO<T> {
    pub code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<T>>,
    pub total: Option<u64>,
    pub msg:Option<String>
}

impl<T> PageVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<Page<T>, Error>) -> Self {
        if arg.is_ok() {
            let arg = arg.as_ref().unwrap();
            Self {
                code: CODE_SUCCESS,
                rows: Some(arg.records.clone()),
                total: Some(arg.total),
                msg: None,
            }
        } else {
            Self {
                code: CODE_FAIL,
                rows: None,
                total: None,
                msg: Some(arg.clone().err().unwrap().to_string()),
            }
        }
    }
    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

impl<T> ToString for PageVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

