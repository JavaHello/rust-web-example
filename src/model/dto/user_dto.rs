use serde::{Deserialize, Serialize};

use crate::util::verify_code::VcType;
use validator::Validate;
use validator_derive::Validate;
/// 登陆

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct SignInDTO {
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

/// 登陆
#[derive(Serialize, Validate, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub username: Option<String>,
    pub name: Option<String>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct VerifyCodeDTO {
    #[validate(required)]
    pub vc_type: Option<VcType>,
    #[serde(default = "verify_code_default_len")]
    pub len: usize,
}

fn verify_code_default_len() -> usize {
    4
}

/// 用户分页
#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct ChangePasswordDTO {
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub old_password: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub new_password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_json() {
        let vc = VerifyCodeDTO {
            vc_type: Some(VcType::REG("zhang".to_owned())),
            len: 4,
        };

        let serialized = serde_json::to_string(&vc).unwrap();

        println!("serialized = {}", serialized);

        let deserialized: VerifyCodeDTO = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(deserialized.len, vc.len);
        assert_eq!(deserialized.vc_type, vc.vc_type);
    }
}
