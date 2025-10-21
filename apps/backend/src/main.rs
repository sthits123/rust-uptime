use axum::{Router, routing::get};


#[tokio::main]
async fn main(){
    // Connect to database
    //Create a simple axum
    let app = Router::new().route("/", get(|| async {"hello , World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

