use axum::routing::{get, put};
use diesel::r2d2::{self, ConnectionManager};
use otus_axum::handlers;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let app_state = Arc::new(otus_axum::AppState { pool });

    let app = axum::Router::new()
        .route(
            "/house",
            get(handlers::list_houses)
                .post(handlers::add_house)
                .delete(handlers::drop_all),
        )
        .route(
            "/houses/{house_id}",
            put(handlers::upd_house).delete(handlers::del_house),
        )
        .route(
            "/houses/{house_id}/rooms",
            get(handlers::get_rooms).post(handlers::add_room),
        )
        .route(
            "/houses/{house_id}/rooms/{room_id}",
            put(handlers::upd_room).delete(handlers::del_room),
        )
        .route(
            "/houses/{house_id}/rooms/{room_id}/devices",
            get(handlers::get_devices).post(handlers::add_device),
        )
        .route(
            "/houses/{house_id}/rooms/{room_id}/devices/{device_id}",
            put(handlers::upd_device).delete(handlers::del_device),
        )
        .with_state(Arc::clone(&app_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server started at http://localhost:3000");
    println!("Run example in terminal");
    println!("> cargo run --example requests ");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
