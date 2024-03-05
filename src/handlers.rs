use axum::{
    body::Body, 
    response::Response,
    extract::{Path, Json},
};

use crate::types;

pub async fn create_transaction(Path(id): Path<String>, Json(tr_req_body): Json<types::CreateTransactionRequest>) -> Response {
    let client_id = match id.parse::<u8>() {
        Ok(id) => id,
        Err(_) => {
            return Response::builder().status(422).body(Body::empty()).unwrap();
        }
    };
    println!("{} -> {:?}", client_id, tr_req_body);
    Response::builder().status(200).body(Body::empty()).unwrap()
}

pub async fn get_statement(Path(id): Path<String>) -> Response {
    let client_id = match id.parse::<u8>() {
        Ok(id) => id,
        Err(_) => {
            return Response::builder().status(422).body(Body::empty()).unwrap();
        }
    };
    println!("{}", client_id);
    Response::builder().status(200).body(Body::empty()).unwrap()
}
