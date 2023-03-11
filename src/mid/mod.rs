use crate::config;
use crate::util::jwt_util::JWTToken;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthExtractorConfig, AuthenticationError,
    },
    middleware::HttpAuthentication,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let secret = &config::BOOT_CONFIG.jwt_secret;
    if let Ok(_) = JWTToken::verify(secret, token) {
        return Ok(req);
    }

    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
    Err((AuthenticationError::from(config).into(), req))
}
