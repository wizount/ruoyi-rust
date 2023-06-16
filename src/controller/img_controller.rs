use std::time::Duration;

use actix_web::{Responder};
use captcha::Captcha;
use captcha::filters::{Dots, Noise, Wave};
use uuid::Uuid;

use crate::config::cache_variables::REDIS_UUID_CAPTCHA;
use crate::domain::vo::{RespJson, RespVO};
use crate::service::CONTEXT;
use crate::util::base64::encode;

pub async fn captcha() -> impl Responder {
    let mut json = RespJson::success();
    json.insert("captchaEnabled".to_string(), CONTEXT.config.captcha_enabled.into());

    if CONTEXT.config.captcha_enabled {
        let id = Uuid::new_v4();
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            .apply_filter(Dots::new(4));
        let png = captcha.as_png().unwrap();
        let captcha_str = captcha.chars_as_string().to_lowercase();
        if CONTEXT.config.debug {
            log::info!(
            "uuid:{},captcha:{}",
            &id.to_string(),
            &captcha_str
        );
        }

        let result = CONTEXT
            .cache_service
            .set_string_ex(
                &format!("{}{}", REDIS_UUID_CAPTCHA, &id.to_string()),
                captcha_str.as_str(),
                Some(Duration::from_secs(
                    CONTEXT.config.captcha_expired_min * 60
                )),
            )
            .await;
        //println!("{:?}", result);
        if CONTEXT.config.debug == false {
            //release mode, return the error
            if result.is_err() {
                return RespVO::from_result(&result).resp_json();
            }
        }
        json.insert("uuid".to_string(), id.to_string().into());
        json.insert("img".to_string(), encode(&png, png.len()).into());
        //  img_vo.uuid = id.to_string();
        //  img_vo.img = encode(&png, png.len());
    }
    json.resp_json()
}
