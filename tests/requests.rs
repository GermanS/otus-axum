#[cfg(test)]
mod crud {
    use otus_axum::{
        handlers::PostRequestDevice,
        models::{Device, House, Room},
    };
    use std::collections::HashMap;

    static HTTP_HOST: &str = "http://localhost:3000";

    #[tokio::test]
    async fn add_device() {
        set_up().await;

        new_device(new_room(new_house().await).await).await;
    }

    async fn set_up() {
        let client = reqwest::Client::new();

        let response = client
            .delete(format!("{}/house", HTTP_HOST))
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
    }

    async fn new_house() -> House {
        let name = "la casa de mi primо";

        let mut data = HashMap::new();
        data.insert("name", name);

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", HTTP_HOST, "house"))
            .json(&data)
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());

        let result = response.json::<House>().await;
        assert!(result.is_ok());

        let house = result.unwrap();

        assert_eq!(&house.name, name);

        house
    }

    async fn new_room(house: House) -> Room {
        let name = "cocina";
        let mut data = HashMap::new();
        data.insert("name", name);

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/houses/{}/rooms", HTTP_HOST, house.id))
            .json(&data)
            .send()
            .await;

        let response = response.unwrap();

        assert!(response.status().is_success());

        let result = response.json::<Room>().await;
        assert!(result.is_ok());

        let room = result.unwrap();

        assert_eq!(&room.name, name);

        room
    }

    async fn new_device(room: Room) -> Device {
        let name = "temperatura en el refrigorico";
        let data = PostRequestDevice {
            name: name.into(),
            state: false,
            device: "termometro".into(),
        };

        let url = format!(
            "{}/houses/{}/rooms/{}/devices",
            HTTP_HOST, room.house, room.id
        );

        let client = reqwest::Client::new();
        let response = client.post(url).json(&data).send().await.unwrap();

        assert!(response.status().is_success());

        let result = response.json::<Device>().await;
        assert!(result.is_ok());

        let room = result.unwrap();

        assert_eq!(&room.name, name);

        room
    }
}
