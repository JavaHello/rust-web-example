use crate::core::Error;
use crate::core::Result;
use crate::model::dto::VerifyCodeDTO;
use crate::util::verify_code::VerifyCode;
use log;
use validator::Validate;

pub struct VerifyCodeService {}

impl VerifyCodeService {
    pub async fn send_reg_code(&self, vc: &VerifyCodeDTO) -> Result<bool> {
        // 参数验证
        vc.validate()?;
        if VerifyCode::has_code(vc.vc_type.as_ref().unwrap()).await? {
            return Err(Error::from("重复发送验证码!"));
        }
        match VerifyCode::gen_code(vc.vc_type.as_ref().unwrap(), vc.len, 3 * 60).await {
            Ok(code) => {
                log::debug!("发送验证码, {:?}, code: {}", &vc.vc_type, code);
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }
}
