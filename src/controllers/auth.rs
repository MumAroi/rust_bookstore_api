use crate::{
    auth::{AuthenticatedUser, Claims},
    entities::{prelude::User, user},
    AppConfig,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::time::SystemTime;

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
    config: &State<AppConfig>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<ResSignIn> {
    let db = db as &DatabaseConnection;

    let u: user::Model = match User::find()
        .filter(user::Column::Email.eq(&req_sign_in.email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Invalid credentials".to_string(),
            )))
        }
    };

    if !verify(&req_sign_in.password, &u.password).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Invalid credentials".to_string(),
        )));
    }

    let claims = Claims {
        sub: u.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((Status::Ok, ResSignIn { token })))
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    id: i32,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[get("/me")]
pub async fn me(db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<Json<ResMe>> {
    let db = db as &DatabaseConnection;

    let u: user::Model = User::find_by_id(user.id).one(db).await?.unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResMe {
            id: u.id,
            email: u.email,
            first_name: u.first_name,
            last_name: u.last_name,
        }),
    )))
}