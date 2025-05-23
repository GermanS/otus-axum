use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{
    AppState,
    models::{Device, House, NewDevice, NewHouse, NewRoom, Room},
    schema,
};

#[derive(Deserialize)]
pub struct Title(String);

impl From<Title> for String {
    fn from(value: Title) -> Self {
        value.0
    }
}
pub async fn list_houses(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    use schema::house::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = house.load::<House>(&mut *dbh).expect("cant execute");

    Json(res)
}

pub async fn add_house(
    State(state): State<Arc<AppState>>,
    Json(title): Json<Title>,
) -> impl IntoResponse {
    use crate::schema::house::dsl::house;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(house)
        .values(NewHouse { name: title.into() })
        .execute(&mut *dbh)
        .expect("cant execute");

    (StatusCode::CREATED, Json(()))
}

pub async fn upd_house(
    State(app_state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
    Json(form): Json<Title>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::house::dsl::*;

    let res = diesel::update(house)
        .filter(id.eq(house_id))
        .set((name.eq::<String>(form.into()),))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}

pub async fn del_house(
    State(app_state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::house::dsl::*;

    let res = diesel::delete(house.filter(id.eq(house_id)))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
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
    Json(title): Json<Title>,
) -> Json<usize> {
    use schema::room::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = diesel::insert_into(room)
        .values(&NewRoom {
            house: house_id,
            name: title.into(),
        })
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res)
}

pub async fn upd_room(
    State(app_state): State<Arc<AppState>>,
    Path(room_id): Path<i32>,
    Json(form): Json<Title>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::room::dsl::*;

    let res = diesel::update(room)
        .filter(id.eq(room_id))
        .set((name.eq::<String>(form.into()),))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}

pub async fn del_room(
    State(app_state): State<Arc<AppState>>,
    Path(room_id): Path<i32>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::room::dsl::*;

    let res = diesel::delete(room.filter(id.eq(room_id)))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
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

#[derive(Deserialize)]
pub struct PostRequestDevice {
    name: String,
    state: bool,
    device: String,
}

pub async fn add_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id)): Path<(i32, i32)>,
    Json(new_device): Json<PostRequestDevice>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = diesel::insert_into(device)
        .values(&NewDevice {
            room: room_id,
            name: new_device.name,
            state: new_device.state,
            device_type: new_device.device,
        })
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}

pub async fn upd_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, _room_id, device_id)): Path<(i32, i32, i32)>,
    Json(form): Json<PostRequestDevice>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = diesel::update(device)
        .filter(id.eq(device_id))
        .set((
            name.eq(form.name),
            state.eq(form.state),
            device_type.eq(form.device),
        ))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}

pub async fn del_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, _room_id, device_id)): Path<(i32, i32, i32)>,
) -> Json<String> {
    let mut dbh = app_state.pool.get().expect("cant connect");

    use schema::device::dsl::*;

    let res = diesel::delete(device.filter(id.eq(device_id)))
        .execute(&mut *dbh)
        .expect("cant execute");

    Json(res.to_string())
}
