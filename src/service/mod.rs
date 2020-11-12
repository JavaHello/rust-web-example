mod cache_service;
mod user_service;
mod verify_code_service;

pub use cache_service::CacheKey;
use cache_service::CacheService;
use user_service::UserService;
use verify_code_service::VerifyCodeService;

use crate::config::BOOT_CONFIG;

lazy_static! {
   ///chache
   pub static ref CACHE_SERVICE: CacheService = CacheService::new(&BOOT_CONFIG.redis_url);

   pub static ref USER_SERVICE: UserService = UserService{};
   pub static ref VERIFY_CODE_SERVICE: VerifyCodeService = VerifyCodeService{};
}
