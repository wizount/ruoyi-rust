use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_http::body::BoxBody;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

use crate::domain::vo::RespVO;
use crate::middleware::auth::{check_auth, checked_token, is_white_list_api};
use crate::util::token::get_token;

pub struct Auth;


impl<S: 'static> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error> + 'static,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let token = get_token(req.request());
        let path = req.path().to_string();
        Box::pin(async move {
            // //debug mode not enable auth
            // if !CONTEXT.config.debug {
                if !is_white_list_api(&path) {
                    //非白名单检查token是否有效
                    match checked_token(&token, &path).await {
                        Ok(data) => {
                            match check_auth(&data, &path).await {
                                Ok(_) => {
                                    //刷新过期时间
                                    crate::web_data::set_user_name(data.user_name);
                                }
                                Err(e) => {
                                    //仅提示拦截
                                    let resp: RespVO<String> = RespVO {
                                        code: 0,
                                        msg: Some(format!("无权限访问:{}", e.to_string())),
                                        data: None,
                                    };
                                    return Ok(req.into_response(resp.resp_json()));
                                }
                            }
                        }
                        Err(e) => {
                            //401 http状态码会强制前端退出当前登陆状态
                            let resp: RespVO<String> = RespVO {
                                code: 401,
                                msg: Some(format!("Unauthorized for:{}", e.to_string())),
                                data: None,
                            };
                            return Ok(req.into_response(resp.resp_json()));//根据前台，需要返回http status code 200，但是code 是401
                        }
                    }
                }
           // }
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
