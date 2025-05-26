CREATE TABLE house (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE room (
    id INTEGER NOT NULL PRIMARY KEY  AUTOINCREMENT,
    house INTEGER NOT NULL REFERENCES house(id),
    name TEXT NOT NULL,

    constraint unique_room_in_house UNIQUE (house, name)
);

CREATE TABLE device (
    id INTEGER NOT NULL PRIMARY KEY  AUTOINCREMENT,
    room INTEGER NOT NULL REFERENCES room(id),
    name TEXT NOT NULL,
    device_type TEXT NOT NULL,
    state BOOLEAN NOT NULL,

    constraint unique_device_in_room UNIQUE (room, name)
);