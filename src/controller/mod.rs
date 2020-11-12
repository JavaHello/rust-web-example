use actix_web::web::{self, ServiceConfig};
pub mod user_controller;
pub mod verify_code_controller;


pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        "/change_password",
        web::post().to(user_controller::change_password),
    )
    .route("/users", web::get().to(user_controller::user_page));
}
