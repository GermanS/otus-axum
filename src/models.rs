use crate::schema::{device, house, room};
use diesel::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = house)]
#[diesel(primary_key(id))]
pub struct House {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = house)]
pub struct NewHouse {
    pub name: String,
}

#[derive(
    serde::Serialize, serde::Deserialize, Queryable, Selectable, Identifiable, Associations, Debug,
)]
#[diesel(table_name = room)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(House, foreign_key=house))]
pub struct Room {
    pub id: i32,
    pub house: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = room)]
pub struct NewRoom {
    pub house: i32,
    pub name: String,
}

#[derive(
    serde::Serialize, serde::Deserialize, Queryable, Selectable, Identifiable, Associations, Debug,
)]
#[diesel(table_name = device)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Room, foreign_key=room))]
pub struct Device {
    pub id: i32,
    pub room: i32,
    pub name: String,
    pub device_type: String,
    pub state: bool,
}

#[derive(Insertable)]
#[diesel(table_name = device)]
pub struct NewDevice {
    pub name: String,
    pub room: i32,
    pub device_type: String,
    pub state: bool,
}
