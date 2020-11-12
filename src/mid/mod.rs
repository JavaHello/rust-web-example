use crate::config;
use crate::util::jwt_util::JWTToken;
use actix_http::error::Error;
use actix_web::dev::ServiceRequest;
use actix_web::HttpResponse;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    let secret = &config::BOOT_CONFIG.jwt_secret;
    if let Ok(_) = JWTToken::verify(secret, token) {
        return Ok(req);
    }
    return Err(Error::from(HttpResponse::Unauthorized()));
}
