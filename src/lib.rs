#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]
extern crate log;
#[macro_use]
extern crate rbatis;
extern crate bcrypt;

#[macro_use]
pub mod util;
pub mod config;
pub mod controller;
pub mod domain;
pub mod error;
//pub mod middleware;
pub mod service;
pub mod web_data;
pub mod http_server;
//管理token，为兼容若依管理平台重新写，不再使用中间件进行验证
pub mod token_auth;