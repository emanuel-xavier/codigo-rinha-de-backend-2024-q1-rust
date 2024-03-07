use axum::{
    body::Body, 
    response::Response,
    extract::{State, Path, Json},
};
use crate::types;

macro_rules! build_response {
    ($status:expr) => {{
        Response::builder()
            .status($status)
            .body(Body::empty())
            .unwrap()
    }};
    ($status:expr, $body:expr) => {{
        Response::builder()
            .status($status)
            .body($body)
            .unwrap()
    }};
}

pub async fn create_transaction(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<String>,
    Json(tr_req_body): Json<types::CreateTransactionRequest>) 
-> Response {
    let client_id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return build_response!(422);
        }
    };

    if !tr_req_body.is_valid() {
        return build_response!(422);
    }

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("begin DB transaction error: {}", err);
            return build_response!(500);
        }
    };

    let mut client = match sqlx::query_as::<_, types::Client>(
        "
            SELECT id, balance, \"limit\" 
            FROM clients 
            WHERE id = $1 
            FOR UPDATE       
        ")
        .bind(client_id)
        .fetch_one(&mut *tx)
        .await
        {
            Ok(client) => client,
            Err(sqlx::Error::RowNotFound) => return build_response!(404),
            Err(err) => {
                eprintln!("select and lock client error: {}", err);
                return build_response!(500);
            }
        };

    match tr_req_body.tr_type {
        types::TransactionType::Debit => {
            client.balance -= tr_req_body.value as i32;
            if client.balance < -client.limit {
                tx.rollback().await.unwrap();
                return build_response!(422);
            }
        },
        types::TransactionType::Credit => {
            client.balance += tr_req_body.value as i32;
        }
    }
      
    match sqlx::query(
        "
            INSERT INTO 
            transaction(value, type, description, client_id)
            VALUES($1, $2, $3, $4)
        ")
        .bind(tr_req_body.value as i32)
        .bind(match tr_req_body.tr_type {
            types::TransactionType::Debit => "d",
            types::TransactionType::Credit => "c",
        })
        .bind(&tr_req_body.description)
        .bind(&client_id)
        .fetch_optional(&mut *tx)
        .await {
            Ok(_) => {},
            Err(err) => {
                eprintln!("create transaction error: {}", err); 
                tx.rollback().await.unwrap();
                return build_response!(500);
            }
        }

    match sqlx::query(
        "
            UPDATE clients
            SET balance = $1
            WHERE id = $2
        ")
        .bind(&client.balance)
        .bind(&client_id)
        .fetch_optional(&mut *tx)
        .await {
            Ok(_) => {},
            Err(err) => {
                eprintln!("update client error: {}", err); 
                tx.rollback().await.unwrap();
                return build_response!(500);
            }
        }
    

    let serialized_body = match serde_json::to_string(
        &types::CreateTransactionResponse {
            limit: client.limit,
            balance: client.balance,
        }
    ) {
        Ok(json_str) => json_str,
        Err(err) => {
            tx.rollback().await.unwrap();
            eprintln!("serialized_body error: {}", err);
            return build_response!(500);
        }
    };

    match tx.commit().await {
        Ok(_) => Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(serialized_body))
                .unwrap(),
        Err(err) => {
            eprintln!("transaciton commit error: {}", err); 
            build_response!(500)
        },

    }

}

pub async fn get_statement(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<String>
) -> Response {
    let client_id = match id.parse::<u8>() {
        Ok(id) => id,
        Err(_) => {
            return Response::builder().status(422).body(Body::empty()).unwrap();
        }
    };
    println!("{}", client_id);
    Response::builder().status(200).body(Body::empty()).unwrap()
}
