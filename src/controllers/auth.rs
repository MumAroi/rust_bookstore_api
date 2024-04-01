use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::DatabaseConnection;

use super::Response;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<ResSignIn> {
    let db = db as &DatabaseConnection;
    todo!()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignUp {
    token: String,
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(db: &State<DatabaseConnection>, req_sign_up: Json<ReqSignUp>) -> Response<String> {
    let db = db as &DatabaseConnection;
    todo!()
}
