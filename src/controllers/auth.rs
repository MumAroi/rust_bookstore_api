use crate::entities::{prelude::User, user};
use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{prelude::DateTimeUtc, *};

use super::{ErrorResponse, Response, SuccessResponse};

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
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignUp {
    token: String,
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    if User::find()
        .filter(user::Column::Email.eq(&req_sign_up.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "An account exists with this email address".to_string(),
        )));
    }

    User::insert(user::ActiveModel {
        email: Set(req_sign_up.email.to_owned()),
        password: Set(hash(req_sign_up.password.to_owned(), DEFAULT_COST).unwrap()),
        first_name: Set(req_sign_up.first_name.to_owned()),
        last_name: Set(req_sign_up.last_name.to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(SuccessResponse((
        Status::Created,
        "Account created".to_string(),
    )))
}
