use super::{
    m20240329_042405_create_users_table::User, m20240329_042432_create_author_table::Author,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Book::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Book::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-user_id")
                            .from(Book::Table, Book::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Book::AuthorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-author_id")
                            .from(Book::Table, Book::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Year).string().not_null())
                    .col(ColumnDef::new(Book::Cover).string().not_null())
                    .col(
                        ColumnDef::new(Book::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Book::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    UserId,
    AuthorId,
    Title,
    Year,
    Cover,
    CreatedAt,
    UpdatedAt,
}
