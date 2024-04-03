use std::time::SystemTime;

use super::{
    books::{ResBook, ResBookList},
    Response, SuccessResponse,
};
use crate::{
    auth::AuthenticatedUser,
    entities::{
        author, book,
        prelude::{Author, Book},
    },
};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryOrder, Set,
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthor {
    id: i32,
    first_name: String,
    last_name: String,
    bio: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorList {
    total: usize,
    authors: Vec<ResAuthor>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqAuthor {
    first_name: String,
    last_name: String,
    bio: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResAuthorList>> {
    let db = db as &DatabaseConnection;

    let authors = Author::find()
        .order_by_desc(author::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|a| ResAuthor {
            id: a.id,
            first_name: a.first_name.to_owned(),
            last_name: a.last_name.to_owned(),
            bio: a.bio.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResAuthorList {
            total: authors.len(),
            authors,
        }),
    )))
}

#[post("/", data = "<req_author>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_author: Json<ReqAuthor>,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let author = author::ActiveModel {
        user_id: Set(user.id),
        first_name: Set(req_author.first_name.to_owned()),
        last_name: Set(req_author.last_name.to_owned()),
        bio: Set(req_author.bio.to_owned()),
        ..Default::default()
    };
    let author = author.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResAuthor {
            id: author.id,
            first_name: author.first_name,
            last_name: author.last_name,
            bio: author.bio,
        }),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let author = Author::find_by_id(id).one(db).await?;

    let author = match author {
        Some(a) => a,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Author not found".to_string(),
            )))
        }
    };

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResAuthor {
            id: author.id,
            first_name: author.first_name,
            last_name: author.last_name,
            bio: author.bio,
        }),
    )))
}

#[put("/<id>", data = "<req_author>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_author: Json<ReqAuthor>,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let mut author: author::ActiveModel = match Author::find_by_id(id).one(db).await? {
        Some(a) => a.into(),
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Author not found".to_string(),
            )))
        }
    };

    author.first_name = Set(req_author.first_name.to_owned());
    author.last_name = Set(req_author.last_name.to_owned());
    author.bio = Set(req_author.bio.to_owned());
    author.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    let author = author.update(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResAuthor {
            id: author.id,
            first_name: author.first_name,
            last_name: author.last_name,
            bio: author.bio,
        }),
    )))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    let author = match Author::find_by_id(id).one(db).await? {
        Some(a) => a,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Author not found".to_string(),
            )))
        }
    };

    author.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Author deleted".to_string())))
}

#[get("/<id>/books")]
pub async fn get_books(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let author = match Author::find_by_id(id).one(db).await? {
        Some(a) => a,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Author not found".to_string(),
            )))
        }
    };

    let books: Vec<book::Model> = author.find_related(Book).all(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books: books
                .into_iter()
                .map(|b| ResBook {
                    id: b.id,
                    author_id: b.author_id,
                    title: b.title,
                    year: b.year,
                    cover: b.cover,
                })
                .collect::<Vec<_>>(),
        }),
    )))
}
