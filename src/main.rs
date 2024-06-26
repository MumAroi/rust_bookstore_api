use controllers::{Response, SuccessResponse};
use dotenv::dotenv;
use fairings::cors::{options, CORS};
use migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::MigratorTrait;

#[macro_use]
extern crate rocket;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("127.0.0.1".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("DB_PASSWORD").unwrap_or("332211".to_string()),
            db_database: std::env::var("DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("Please set the secret key in the .env file"),
        }
    }
}

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello, world!".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let config: AppConfig = AppConfig::default();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    match Migrator::up(&db, None).await {
        Err(err) => panic!("{}", err),
        Ok(_) => (),
    };

    rocket::build()
        .attach(CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![
                controllers::auth::sign_in,
                controllers::auth::sign_up,
                controllers::auth::me
            ],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
                controllers::authors::get_books,
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete
            ],
        )
}
