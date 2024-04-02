use super::{Response, SuccessResponse};
use crate::{
    auth::AuthenticatedUser,
    entities::{author, prelude::Author},
};
use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
    State,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReAuthor {
    id: i32,
    first_name: String,
    last_name: String,
    bio: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReAuthorList {
    total: usize,
    authors: Vec<ReAuthor>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ReAuthorList>> {
    let db = db as &DatabaseConnection;

    let authors = Author::find()
        .order_by_desc(author::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|a| ReAuthor {
            id: a.id,
            first_name: a.first_name.to_owned(),
            last_name: a.last_name.to_owned(),
            bio: a.bio.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ReAuthorList {
            total: authors.len(),
            authors,
        }),
    )))
}

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn show(id: u32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
