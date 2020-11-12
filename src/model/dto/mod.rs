use serde::{Deserialize, Serialize};

use validator::Validate;
use validator_derive::Validate;

mod user_dto;

pub use user_dto::ChangePasswordDTO;
pub use user_dto::SignInDTO;
pub use user_dto::UserAddDTO;
pub use user_dto::UserPageDTO;
pub use user_dto::VerifyCodeDTO;

/// 资源分页DTO
#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct ReqPageDTO {
    #[validate(required)]
    pub page: Option<u64>,
    #[validate(required)]
    pub size: Option<u64>,
    pub user_no: Option<String>,
}
