mod routes;

use axum::{
    Router,
    Server
};


#[tokio::main]
async fn main() {
    let app: Router = routes::api_routes();
    println!("Server Running on port 3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
