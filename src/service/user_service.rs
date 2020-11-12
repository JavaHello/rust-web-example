use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use rbatis::wrapper::Wrapper;

use crate::config::BOOT_CONFIG;
use crate::core::Error;
use crate::core::Result;
use crate::dao::RB;
use crate::model::domain::TmUser;
use crate::model::dto::ChangePasswordDTO;
use crate::model::dto::SignInDTO;
use crate::model::dto::UserAddDTO;
use crate::model::dto::UserPageDTO;
use crate::model::vo::SignInVO;
use crate::service::{CacheKey, CACHE_SERVICE};
use crate::util::jwt_util::JWTToken;
use crate::util::password_encoder::PasswordEncoder;
use crate::util::rand_util::RandUtil;
use crate::util::verify_code::{VcType, VerifyCode};
use validator::Validate;
/// 用户服务
pub struct UserService {}

impl UserService {
    ///登陆后台
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        arg.validate()?;
        // 验证验证码
        if !VerifyCode::verify(
            &VcType::SignIn(arg.username.as_ref().unwrap().clone()),
            arg.verify_code.as_ref().unwrap(),
        )
        .await
        {
            return Err(Error::from("验证码错误!"));
        }
        let w = Wrapper::new(&RB.driver_type()?)
            .eq("username", &arg.username)
            .check()?;
        let user: Option<TmUser> = RB.fetch_by_wrapper("", &w).await?;
        if user.is_none() {
            return Err(Error::from(format!(
                "用户:{} 不存在!",
                arg.username.as_ref().unwrap()
            )));
        }
        let user = user.unwrap();
        // check pwd
        if !PasswordEncoder::verify(
            user.password.as_ref().unwrap(),
            arg.password.as_ref().unwrap(),
            user.salt.as_ref().unwrap(),
        ) {
            return Err(Error::from("密码不正确!"));
        }
        let jwt = JWTToken::new(user.username.as_ref().unwrap());
        // let secret = RandUtil::rand_code(32);
        let secret = &BOOT_CONFIG.jwt_secret;
        CACHE_SERVICE
            .put_string(
                &CacheKey::JwtSecret(user.username.as_ref().unwrap().clone()),
                &secret,
            )
            .await?;
        if let Ok(token) = jwt.create_token(&secret) {
            let sign_vo = SignInVO {
                user_no: user.user_no,
                username: user.username,
                head_icon: user.head_icon,
                token: Some(token),
            };
            return Ok(sign_vo);
        }
        return Err(Error::from("登录失败"));
    }
    ///登陆后台
    pub async fn add(&self, arg: &UserAddDTO) -> Result<u64> {
        // 参数验证
        arg.validate()?;
        // 验证验证码
        if !VerifyCode::verify(
            &VcType::REG(arg.username.as_ref().unwrap().clone()),
            arg.verify_code.as_ref().unwrap(),
        )
        .await
        {
            return Err(Error::from("验证码错误!"));
        }

        let w = Wrapper::new(&RB.driver_type()?)
            .eq("username", &arg.username)
            .check()?;
        let user: Option<TmUser> = RB.fetch_by_wrapper("", &w).await?;
        match user {
            Some(_) => {
                return Err(Error::from("用户已存在!"));
            }
            _ => (),
        };
        let mut user = TmUser::new();
        user.username = arg.username.clone();
        user.user_no = Some(RandUtil::rand_code(32));
        let salt = RandUtil::rand_code(32);
        user.password = Some(PasswordEncoder::encode(
            arg.password.as_ref().unwrap(),
            &salt,
        ));
        user.salt = Some(salt);
        let res = RB.save("", &user).await?;
        Ok(res.rows_affected)
    }

    pub async fn change_password(&self, arg: &ChangePasswordDTO) -> Result<bool> {
        arg.validate()?;
        // 验证验证码
        if !VerifyCode::verify(
            &VcType::ChangePassword(arg.username.as_ref().unwrap().clone()),
            arg.verify_code.as_ref().unwrap(),
        )
        .await
        {
            return Err(Error::from("验证码错误!"));
        }
        let w = Wrapper::new(&RB.driver_type()?)
            .eq("username", &arg.username)
            .check()?;
        let user: Option<TmUser> = RB.fetch_by_wrapper("", &w).await?;
        if user.is_none() {
            return Err(Error::from(format!(
                "用户:{} 不存在!",
                arg.username.as_ref().unwrap()
            )));
        }
        let mut user = user.unwrap();
        // check pwd
        if !PasswordEncoder::verify(
            user.password.as_ref().unwrap(),
            arg.old_password.as_ref().unwrap(),
            user.salt.as_ref().unwrap(),
        ) {
            return Err(Error::from("密码不正确!"));
        }
        let salt = RandUtil::rand_code(32);
        user.password = Some(PasswordEncoder::encode(
            arg.new_password.as_ref().unwrap(),
            &salt,
        ));
        user.salt = Some(salt);
        let i = RB.update_by_wrapper("", &user, &w, true).await?;
        return Ok(i == 1);
    }

    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<TmUser>> {
        let w = Wrapper::new(&RB.driver_type()?);
        let page_req = PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10));
        let data: Page<TmUser> = RB.fetch_page_by_wrapper("", &w, &page_req).await?;
        Ok(data)
    }
}
