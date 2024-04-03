use std::time::SystemTime;

use super::{Response, SuccessResponse};
use crate::{
    auth::AuthenticatedUser,
    entities::{book, prelude::Book},
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
pub struct ResBook {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    pub total: usize,
    pub books: Vec<ResBook>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|book| ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title.to_owned(),
            year: book.year.to_owned(),
            cover: book.cover.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id),
        author_id: Set(req_book.author_id.to_owned()),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
        }),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(b) => b,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Book not found".to_string(),
            )))
        }
    };

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
        }),
    )))
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let mut book: book::ActiveModel = match Book::find_by_id(id).one(db).await? {
        Some(b) => b.into(),
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Book not found".to_string(),
            )))
        }
    };

    book.author_id = Set(req_book.author_id);
    book.title = Set(req_book.title.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.cover = Set(req_book.cover.to_owned());
    book.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    let book = book.update(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
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

    let book = match Book::find_by_id(id).one(db).await? {
        Some(b) => b,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Book not found".to_string(),
            )))
        }
    };
    
    book.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Book deleted".to_string())))
}
