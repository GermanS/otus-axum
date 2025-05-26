static HTTP_HOST: &str = "http://localhost:3000";

#[cfg(test)]
mod house_tests {
    use std::collections::HashMap;

    use crate::HTTP_HOST;

    #[tokio::test]
    async fn add_a_house() {
        let mut data = HashMap::new();
        data.insert("name", "la casa de mi primo");

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", HTTP_HOST, "house"))
            .json(&data)
            .send()
            .await
            .unwrap();

        println!("{:?}", response);

        assert!(response.status().is_success());
    }


}
