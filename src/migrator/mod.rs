pub use sea_orm_migration::prelude::*;

mod m20240329_042405_create_users_table;
mod m20240329_042432_create_author_table;
mod m20240329_042458_create_book_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240329_042405_create_users_table::Migration),
            Box::new(m20240329_042432_create_author_table::Migration),
            Box::new(m20240329_042458_create_book_table::Migration),
        ]
    }
}
