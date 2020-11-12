use crate::model::dto::{ChangePasswordDTO, SignInDTO, UserAddDTO, UserPageDTO};
use crate::model::vo::RespVO;
use crate::service::USER_SERVICE;
use actix_web::{web, Responder};

/// 用户登陆
pub async fn user_login(arg: web::Json<SignInDTO>) -> impl Responder {
    let vo = USER_SERVICE.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).to_json_resp();
}

/// 用户添加
pub async fn user_reg(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = USER_SERVICE.add(&arg.0).await;
    return RespVO::from_result(&vo).to_json_resp();
}

/// 修改密码
pub async fn change_password(arg: web::Json<ChangePasswordDTO>) -> impl Responder {
    let vo = USER_SERVICE.change_password(&arg.0).await;
    return RespVO::from_result(&vo).to_json_resp();
}

///用户分页
pub async fn user_page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = USER_SERVICE.page(&arg.0).await;
    return RespVO::from_result(&vo).to_json_resp();
}
