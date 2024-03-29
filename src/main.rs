#[macro_use] extern crate rocket;

mod migrator;
mod db;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("127.0.0.1".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap_or("root".to_string()), 
            db_password: std::env::var("DB_PASSWORD").unwrap_or("332211".to_string()), 
            db_database: std::env::var("DB_DATABASE").unwrap_or("bookstore".to_string()), 
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
