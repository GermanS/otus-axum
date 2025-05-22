mod models;
mod schema;

use anyhow::{Ok, Result};
use axum::{Extension, Json, Router, extract::Path, http::StatusCode, routing::get};
use diesel::{
    Insertable, RunQueryDsl, SelectableHelper, SqliteConnection,
    associations::HasTable,
    r2d2::{self, ConnectionManager, Pool},
    result,
};
use models::{Device, House, NewDevice, NewHouse, Room};
use std::sync::Arc;

struct AppState {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/houses", get(list_houses))
        //.route("/houses/:house_id/rooms", get(get_rooms).post(add_room))
        //.route("/houses/:house_id/rooms/:room_id/devices", get(get_devices))
        .layer(Extension(app_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    println!("Сервер запущен на http://localhost:3000");

    Ok(())
}

async fn list_houses(Extension(state): Extension<Arc<AppState>>) -> Json<Vec<House>> {
    use schema::house::dsl::*;

    let mut conn = state.pool.get().expect("cant connect");

    let results = house.load::<House>(&mut *conn).expect("cant execute");

    Json(results)
}

async fn create_house(
    Extension(state): Extension<Arc<AppState>>,
    Json(new_house): Json<NewHouse>,
) -> Json<usize> {
    use schema::house::dsl::house;

    let mut conn = state.pool.get().expect("cant connect");

    let results = diesel::insert_into(house::table())
        .values(new_house)
        .execute(&mut *conn)
        .expect("cant execute");

    Json(results)
}

async fn get_rooms(Path(ousex): Path<String>) -> Json<Vec<String>> {
    // Запрос комнат из БД
    //Json(vec!["Kitchen".to_string(), "Bedroom".to_string()])
    todo!()
}

async fn add_room(Path(house_id): Path<String>, Json(name): Json<String>) -> Json<String> {
    // Добавление комнаты в дом
    // Json(format!("Комната '{}' добавлена в дом '{}'", name, house_id))
    todo!()
}

async fn get_devices(Path((house_id, room_id)): Path<(String, String)>) -> Json<Vec<String>> {
    // Получаем список устройств из БД
    // Json(vec!["TV Socket", "Termometer"].map(|s| s.to_string()))
    todo!()
}

async fn add_device(
    Path((house_id, room_id)): Path<(String, String)>,
    Json(payload): Json<NewDevice>,
) -> Json<String> {
    // Сохраняем устройство в БД
    // Json(format!(
    //     "Устройство '{}' добавлено в комнату '{}' дома '{}'",
    //     payload.name, room_id, house_id
    // ))
    todo!()
}
