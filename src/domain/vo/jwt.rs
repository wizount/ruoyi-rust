use crate::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT authentication Token structure
/// diff: 不再保存如此多的信息，都保留在缓存
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JWTToken {
    //账号id
    pub login_user_key: String,
    //过期时间
    pub exp: usize,
}

impl JWTToken {
    /// create token_auth
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token_auth invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation::default();
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}
