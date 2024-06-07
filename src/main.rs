use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


#[derive(Deserialize)]

//Estrutura do BODY do post
struct Transaction {
    name: String,
}

#[derive(Serialize)]
//Estrutura da saída do post
struct Output {
    message: String,
}

async fn handle_post(Json(payload): Json<Transaction>) -> Json<Output> {
    //Aqui você faz o que você quiser com o BODY do post
    let response = Output {
        message: format!("Hello, {}!", payload.name),
    };
    Json(response)
}

async fn handle_get() -> Json<bool> {
    Json(true)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_get))
        .route("/post", post(handle_post));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


