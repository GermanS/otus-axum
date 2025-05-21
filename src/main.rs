mod models;
mod schema;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post},
};
use diesel::prelude::*;
use models::{Device, NewDevice, Room};
use std::sync::Arc;

struct AppState {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/houses", get(list_houses).post(create_house))
        .route("/houses/:house_id/rooms", get(get_rooms).post(add_room))
        .route(
            "/houses/:house_id/rooms/:room_id/devices",
            get(get_devices).post(add_device),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Сервер запущен на http://localhost:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn list_houses() -> Json<Vec<String>> {
    // Запрос к базе данных для получения всех домов
    Json(vec!["My Smart House".to_string()])
}

async fn create_house(Json(payload): Json<String>) -> Json<String> {
    // Сохранение нового дома в базу данных
    Json(format!("Дом '{}' создан", payload))
}

async fn get_rooms(Path(house): Path<String>) -> Json<Vec<String>> {
    // Запрос комнат из БД
    Json(vec!["Kitchen".to_string(), "Bedroom".to_string()])
}

async fn add_room(Path(house_id): Path<String>, Json(name): Json<String>) -> Json<String> {
    // Добавление комнаты в дом
    Json(format!("Комната '{}' добавлена в дом '{}'", name, house_id))
}

async fn get_devices(Path((house_id, room_id)): Path<(String, String)>) -> Json<Vec<String>> {
    // Получаем список устройств из БД
    Json(vec!["TV Socket", "Termometer"].map(|s| s.to_string()))
}

async fn add_device(
    Path((house_id, room_id)): Path<(String, String)>,
    Json(payload): Json<NewDevice<'_>>,
) -> Json<String> {
    // Сохраняем устройство в БД
    Json(format!(
        "Устройство '{}' добавлено в комнату '{}' дома '{}'",
        payload.name, room_id, house_id
    ))
}
