use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    AppState,
    models::{Device, House, NewDevice, NewHouse, NewRoom, Room},
    schema,
};

pub async fn list_houses(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    use schema::house::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = house.load::<House>(&mut *dbh).expect("cant execute");

    Json(res)
}

pub async fn add_house(
    State(state): State<Arc<AppState>>,
    Json(name): Json<String>,
) -> impl IntoResponse {
    use crate::schema::house::dsl::house;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(house)
        .values(NewHouse { name })
        .execute(&mut *dbh)
        .expect("cant execute");

    (StatusCode::CREATED, Json(()))
}

pub async fn get_rooms(
    State(state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
) -> Json<Vec<Room>> {
    use crate::schema::room::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = room
        .filter(house.eq(house_id))
        .load::<Room>(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

pub async fn add_room(
    State(state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
    Json(new_room): Json<String>,
) -> Json<usize> {
    use schema::room::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(room)
        .values(&NewRoom {
            house: house_id,
            name: new_room,
        })
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

pub async fn get_devices(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id)): Path<(i32, i32)>,
) -> Json<Vec<Device>> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = device
        .filter(room.eq(room_id))
        .load::<Device>(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

pub async fn add_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id)): Path<(i32, i32)>,
    Json(new_device): Json<String>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = diesel::insert_into(device)
        .values(&NewDevice {
            room: room_id,
            name: new_device,
            state: false,
            device_type: "Socket".into(),
        })
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}
