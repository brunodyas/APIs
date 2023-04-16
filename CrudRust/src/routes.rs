use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db;
use crate::models::{User, NewUser};

// Handler functions

async fn index() -> impl Responder {
    HttpResponse::Ok().body("API is running!")
}

async fn get_users() -> impl Responder {
    let users = db::list_users().await;
    HttpResponse::Ok().json(users)
}

async fn get_user_by_id(path: web::Path<(u32,)>) -> impl Responder {
    let user_id = path.0;
    let user = db::get_user_by_id(user_id).await;
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found!"),
    }
}

async fn add_user(user: web::Json<NewUser>) -> impl Responder {
    let new_user = User {
        id: 0,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
    };
    let added_user = db::add_user(new_user).await;
    HttpResponse::Created().json(added_user)
}

async fn update_user(path: web::Path<(u32,)>, user: web::Json<User>) -> impl Responder {
    let user_id = path.0;
    let updated_user = db::update_user(user_id, user.name.to_owned(), user.email.to_owned()).await;
    match updated_user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found!"),
    }
}

async fn delete_user(path: web::Path<(u32,)>) -> impl Responder {
    let user_id = path.0;
    let deleted_user = db::delete_user(user_id).await;
    match deleted_user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found!"),
    }
}

// Routes

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    ).service(
        web::resource("/users")
            .route(web::get().to(get_users))
            .route(web::post().to(add_user))
    ).service(
        web::resource("/users/{id}")
            .route(web::get().to(get_user_by_id))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
    );
}
