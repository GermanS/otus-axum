use diesel::r2d2::{ConnectionManager, Pool};

pub mod handlers;
pub mod models;
pub mod schema;

type DbPool = Pool<ConnectionManager<diesel::SqliteConnection>>;

pub struct AppState {
    pub pool: DbPool,
}
