#[cfg(test)]
mod crud {
    use axum::http::response;
    use otus_axum::models::{Device, House, Room};
    use std::collections::HashMap;

    static HTTP_HOST: &str = "http://localhost:3000";

    #[tokio::test]
    async fn add_a_house() {
        set_up().await;

        new_house().await;
    }

    #[tokio::test]
    async fn add_room() {
        set_up().await;

        let house = new_house().await;

        new_room(house).await;
    }

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
        let mut data = HashMap::new();
        data.insert("name", "la casa de mi primо");

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

        result.unwrap()
    }

    async fn new_room(house: House) -> Room {
        let mut data = HashMap::new();
        data.insert("name", "cocina");

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/houses/{}/rooms", HTTP_HOST, house.id))
            .json(&data)
            .send()
            .await;

        let response = response.unwrap();

        assert!(response.status().is_success());

        let result = response.json::<Room>().await;
        println!("{:?}", result);

        assert!(result.is_ok());

        result.unwrap()
    }

    async fn new_device(room: Room) -> Device {
        let mut data = HashMap::new();
        data.insert("name", "cocina");

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}/room/{}", HTTP_HOST, room.house, room.id))
            .json(&data)
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());

        let result = response.json::<Device>().await;
        assert!(result.is_ok());

        result.unwrap()
    }
}
