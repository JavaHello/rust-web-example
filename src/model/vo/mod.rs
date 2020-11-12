use crate::core::Error;
use crate::core::Result;
use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod user_vo;

pub use user_vo::SignInVO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T>) -> Self {
        match arg {
            Ok(data) => Self {
                code: Some("S".to_owned()),
                msg: None,
                data: Some(data.clone()),
            },
            Err(e) => Self {
                code: Some("F".to_owned()),
                msg: Some(e.to_string()),
                data: None,
            },
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some("S".to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn to_json_resp(&self) -> actix_http::Response {
        return HttpResponse::Ok()
            .content_type("json")
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
