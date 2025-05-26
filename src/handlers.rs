use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::{
    AppState,
    models::{Device, House, NewDevice, NewHouse, NewRoom, Room},
    schema::{self, house::name},
};

#[derive(Deserialize, Debug)]
pub struct HouseForm {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct RoomForm {
    name: String,
}

pub async fn list_houses(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    use schema::house::dsl::*;

    let mut dbh = state.pool.get().expect("cant connect");

    let res = house.load::<House>(&mut *dbh).expect("cant execute");

    Json(res)
}

pub async fn add_house(
    State(state): State<Arc<AppState>>,
    Json(house_form): Json<HouseForm>,
) -> Result<Json<House>, (StatusCode, String)> {
    use crate::schema::house::dsl::house;

    let mut dbh = state.pool.get().map_err(internal_error)?;

    let _ = diesel::insert_into(house)
        .values(NewHouse {
            name: house_form.name.clone(),
        })
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = house
        .filter(name.eq(house_form.name))
        .select(House::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn upd_house(
    State(app_state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
    Json(house_form): Json<HouseForm>,
) -> Result<Json<House>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::house::dsl::*;

    let _ = diesel::update(house)
        .filter(id.eq(house_id))
        .set(name.eq::<String>(house_form.name))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = house
        .filter(id.eq(house_id))
        .select(House::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn del_house(
    State(app_state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::house::dsl::*;

    let res = diesel::delete(house.filter(id.eq(house_id)))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    Ok(Json(res.to_string()))
}

pub async fn get_rooms(
    State(state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    use crate::schema::room::dsl::*;

    let mut dbh = state.pool.get().map_err(internal_error)?;

    let res = room
        .filter(house.eq(house_id))
        .load::<Room>(&mut *dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn add_room(
    State(state): State<Arc<AppState>>,
    Path(house_id): Path<i32>,
    Json(room_form): Json<RoomForm>,
) -> Result<Json<Room>, (StatusCode, String)> {
    use schema::room::dsl::*;

    let mut dbh = state.pool.get().map_err(internal_error)?;

    let room_name = room_form.name;

    let _ = diesel::insert_into(room)
        .values(&NewRoom {
            house: house_id,
            name: room_name.to_owned(),
        })
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = room
        .filter(house.eq(house_id))
        .filter(name.eq(room_name.to_owned()))
        .select(Room::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn upd_room(
    State(app_state): State<Arc<AppState>>,
    Path((house_id, room_id)): Path<(i32, i32)>,
    Json(room_form): Json<RoomForm>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    let room_name = room_form.name;

    use schema::room::dsl::*;

    let _ = diesel::update(room)
        .filter(id.eq(room_id))
        .set(name.eq(room_name.to_owned()))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = room
        .filter(house.eq(house_id))
        .filter(name.eq(room_name.to_owned()))
        .select(Room::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn del_room(
    State(app_state): State<Arc<AppState>>,
    Path(room_id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::room::dsl::*;

    let res = diesel::delete(room.filter(id.eq(room_id)))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    Ok(Json(res.to_string()))
}

pub async fn get_devices(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Device>>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::device::dsl::*;

    let res = device
        .filter(room.eq(room_id))
        .load::<Device>(&mut *dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

#[derive(Deserialize, serde::Serialize, Debug)]
pub struct PostRequestDevice {
    pub name: String,
    pub state: bool,
    pub device: String,
}

pub async fn add_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id)): Path<(i32, i32)>,
    Json(new_device): Json<PostRequestDevice>,
) -> Result<Json<Device>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::device::dsl::*;

    let dev_name = new_device.name;

    let _ = diesel::insert_into(device)
        .values(&NewDevice {
            room: room_id,
            name: dev_name.to_owned(),
            //state: new_device.state,
            state: false,
            device_type: new_device.device,
        })
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = device
        .filter(room.eq(room_id))
        .filter(name.eq(dev_name.to_owned()))
        .select(Device::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn upd_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id, device_id)): Path<(i32, i32, i32)>,
    Json(form): Json<PostRequestDevice>,
) -> Result<Json<Device>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::device::dsl::*;

    let dev_name = form.name;

    let _ = diesel::update(device)
        .filter(id.eq(device_id))
        .filter(room.eq(room_id))
        .set((
            name.eq(dev_name.to_owned()),
            state.eq(form.state),
            device_type.eq(form.device),
        ))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    let res = device
        .filter(id.eq(device_id))
        .filter(room.eq(room_id))
        .filter(name.eq(dev_name.to_owned()))
        .select(Device::as_select())
        .first(&mut dbh)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn del_device(
    State(app_state): State<Arc<AppState>>,
    Path((_house_id, room_id, device_id)): Path<(i32, i32, i32)>,
) -> Result<Json<String>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    use schema::device::dsl::*;

    let res = diesel::delete(device.filter(id.eq(device_id)).filter(room.eq(room_id)))
        .execute(&mut *dbh)
        .map_err(internal_error)?;

    Ok(Json(res.to_string()))
}

pub async fn drop_all(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let mut dbh = app_state.pool.get().map_err(internal_error)?;

    {
        use schema::device::dsl::*;

        let _ = diesel::delete(device)
            .execute(&mut *dbh)
            //.expect("cant execute");
            .map_err(internal_error)?;
    }
    {
        use schema::room::dsl::*;

        let _ = diesel::delete(room)
            .execute(&mut *dbh)
            .map_err(internal_error)?;
    }

    {
        use schema::house::dsl::*;

        let _ = diesel::delete(house)
            .execute(&mut *dbh)
            .map_err(internal_error)?;
    }

    Ok(Json(true))
}

fn internal_error<E>(error: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
