use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;
use std::{env, io};

#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../error.rs"]
mod error;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use crate::error::WebServiceError;
use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                WebServiceError::InvalidInput("Invalid input".to_string()).into()
            }))
            .wrap(cors)
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
