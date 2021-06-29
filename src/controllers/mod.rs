use actix_web::{web, Scope};

pub mod users;

pub fn get_all_routers() -> Scope {
    web::scope("").service(users::get_routers())
}
