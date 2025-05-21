use diesel::prelude::*;
use crate::schema::{house, room, device};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = house)]
#[diesel(primary_key(id))]
pub struct House {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
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

#[derive(Queryable, Selectable, Identifiable, Associations)]
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