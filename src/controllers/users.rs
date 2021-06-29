use crate::models::users;
use actix_web::{web, HttpResponse, Responder, Scope};

pub async fn get_one(web::Path(id): web::Path<String>) -> impl Responder {
    let result = users::get_one(&id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            println!("Error on insert: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all() -> impl Responder {
    let result = users::get_all().await;
    HttpResponse::Ok().json(result)
}

pub async fn post(data: web::Json<users::User>) -> impl Responder {
    let result = users::insert_one(&data.name, data.age).await;

    match result {
        Ok(document) => HttpResponse::Ok().json(document),
        Err(e) => {
            println!("Error on insert: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn get_routers() -> Scope {
    web::scope("/user")
        .route("", web::get().to(get_all))
        .route("", web::post().to(post))
        .route("/{id}", web::get().to(get_one))
}
