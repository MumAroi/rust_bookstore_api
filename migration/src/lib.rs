pub use sea_orm_migration::prelude::*;

mod m20240328_145206_create_users_table;
mod m20240329_024433_create_author_table;
mod m20240329_031446_create_book_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240328_145206_create_users_table::Migration),
            Box::new(m20240329_024433_create_author_table::Migration),
            Box::new(m20240329_031446_create_book_table::Migration),
        ]
    }
}
