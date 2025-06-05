pub use axum::{
    Json, Router,
    routing::{get, post},
};

pub fn run_http_server(app: Router, port: &str) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let add = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(&add).await.unwrap();

            println!("Listening on {}", add);
            axum::serve(listener, app).await.unwrap();
        });
}
