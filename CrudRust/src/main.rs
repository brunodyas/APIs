use actix_web::{App, HttpServer};

mod db;
mod models;
mod handlers;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::create_db_pool().await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(
                routes::create_user
                    .to(|| handlers::create_user()),
            )
            .service(
                routes::get_user
                    .to(|| handlers::get_user()),
            )
            .service(
                routes::get_users
                    .to(|| handlers::get_users()),
            )
            .service(
                routes::update_user
                    .to(|| handlers::update_user()),
            )
            .service(
                routes::delete_user
                    .to(|| handlers::delete_user()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
