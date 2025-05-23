mod handlers;
mod models;
mod schema;

use axum::{Router, routing::get};
use diesel::r2d2::{self, ConnectionManager, Pool};
use std::sync::Arc;

type DbPool = Pool<ConnectionManager<diesel::SqliteConnection>>;

struct AppState {
    pool: DbPool,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route(
            "/house",
            get(handlers::list_houses).post(handlers::add_house),
        )
        .route(
            "/houses/:house_id/rooms",
            get(handlers::get_rooms).post(handlers::add_room),
        )
        .route(
            "/houses/:house_id/rooms/:room_id/devices",
            get(handlers::get_devices).post(handlers::add_device),
        )
        .with_state(Arc::clone(&app_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Сервер запущен на http://localhost:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
