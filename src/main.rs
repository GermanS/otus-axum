use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/room", get(get_rooms));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Hello world"
}

async  fn get_rooms() -> &'static str {
    "rooms"
}
