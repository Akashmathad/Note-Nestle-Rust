pub use sea_orm_migration::prelude::*;

mod m20241029_105944_create_subjects;
mod m20241103_005431_subject_changes;
mod m20241103_073205_user_table;
mod m20241103_100024_feedback_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241029_105944_create_subjects::Migration),
            Box::new(m20241103_005431_subject_changes::Migration),
            Box::new(m20241103_073205_user_table::Migration),
            Box::new(m20241103_100024_feedback_table::Migration),
        ]
    }
}
