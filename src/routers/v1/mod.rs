use actix_web::web;

mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(users::login_user)
    );
}
