use std::net::SocketAddr;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

use tracing_subscriber;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // build our application with a route
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback_service(ServeFile::new("static/index.html"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
