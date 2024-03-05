use axum::{
    routing::{get, post},
    Router,
};
mod handlers;
mod types;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/clientes/:id/extrato", get(handlers::get_statement))
        .route("/clientes/:id/transacoes", post(handlers::create_transaction));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
