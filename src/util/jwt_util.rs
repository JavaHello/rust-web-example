use crate::core::Error;
use crate::core::Result;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    id: String,
    username: String,
    aud: String, // (audience)：受众
    exp: usize,
    iat: usize,  // (Issued At)：签发时间
    iss: String, // (issuer)：签发人
    nbf: usize,  // (Not Before)：生效时间
    sub: String, // (subject)：主题
    jti: String, // (JWT ID)：编号
}

impl JWTToken {
    pub fn new(username: &str) -> JWTToken {
        let now = SystemTime::now();
        // 30 分钟过期时间
        let m30 = Duration::from_secs(30 * 60);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");
        JWTToken {
            id: String::from("abc"),
            username: String::from(username),
            aud: String::from("s: &str"), // (audience)：受众
            exp: (now + m30).as_secs() as usize,
            iat: now.as_secs() as usize,  // (Issued At)：签发时间
            iss: String::from("luokai"),     // (issuer)：签发人
            nbf: now.as_secs() as usize,  // (Not Before)：生效时间
            sub: String::from("luokai.ltd"), // (subject)：主题
            jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                ErrorKind::ExpiredSignature => return Err(Error::from("ExpiredSignature")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::util::jwt_util::*;
    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new("username: &str");
        let res = jwt.create_token("test");
        let res = res.unwrap();
        let token = JWTToken::verify("test", &res);
        assert!(token.is_ok());
    }
}
