use crate::model::dto::VerifyCodeDTO;
use crate::model::vo::RespVO;
use crate::service::VERIFY_CODE_SERVICE;
use actix_web::{web, Responder};

/// 获取验证码
pub async fn verify_code(arg: web::Json<VerifyCodeDTO>) -> impl Responder {
    let vo = VERIFY_CODE_SERVICE.send_reg_code(&arg.0).await;
    return RespVO::from_result(&vo).to_json_resp();
}
