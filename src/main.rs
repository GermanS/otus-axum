mod models;
mod schema;

use anyhow::{Ok, Result};
use axum::{
    Extension, Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection,
    associations::HasTable,
    query_dsl::methods::FilterDsl,
    r2d2::{self, ConnectionManager, Pool},
};
use models::{Device, House, NewDevice, NewHouse, NewRoom, Room};
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
        .route("/houses", get(list_houses).post(add_house))
        .route("/houses/:house_id/rooms", get(get_rooms).post(add_room))
        .route(
            "/houses/:house_id/rooms/:room_id/devices",
            get(get_devices).post(add_device),
        )
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

    let mut dbh = state.pool.get().expect("cant connect");

    let res = house.load::<House>(&mut *dbh).expect("cant execute");

    Json(res)
}

async fn add_house(
    Extension(state): Extension<Arc<AppState>>,
    Json(new_house): Json<NewHouse>,
) -> Json<usize> {
    use schema::house::dsl::house;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(house::table())
        .values(new_house)
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

async fn get_rooms(
    Extension(state): Extension<Arc<AppState>>,
    Path(house_id): Path<i32>,
) -> Json<Vec<Room>> {
    use schema::room::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = room
        .filter(house.eq(house_id))
        .load::<Room>(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

async fn add_room(
    Extension(state): Extension<Arc<AppState>>,
    Json(new_room): Json<NewRoom>,
) -> Json<usize> {
    use schema::room::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(room::table())
        .values(new_room)
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

async fn get_devices(
    Path((house_id, room_id)): Path<(i32, i32)>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Json<Vec<Device>> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = device
        .filter(room.eq(room_id))
        .load::<Device>(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

async fn add_device(
    Extension(app_state): Extension<Arc<AppState>>,
    //../Path((house_id, room_id)): Path<(String, String)>,
    Json(new_device): Json<NewDevice>,
) -> Json<usize> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = diesel::insert_into(device::table())
        .values(new_device)
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res)
}
