use axum::{
    routing::{get, post},
    Router,
};
use std::env;
use sqlx::postgres::PgPoolOptions;
mod handlers;
mod types;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let connection_string = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://postgres:1234@localhost:5432/postgres"));

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&connection_string)
        .await
        .expect("Failed to create postgres connection pool");

    let app = Router::new()
        .route("/clientes/:id/extrato", get(handlers::get_statement))
        .route("/clientes/:id/transacoes", post(handlers::create_transaction))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
