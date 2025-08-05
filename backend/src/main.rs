use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

use crate::{auth::client::{oauth}, routes::user};

mod auth;
mod routes;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let url = env::var("DATABASE_URL").expect("Where the fuck is the connection string of the db");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&url)
        .await
        .expect("Could not connect to DB");


    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(user::insert)
        .service(user::update)
        .service(auth::route::login)
        .service(auth::route::callback)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
