use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
enum TransactionType {
    #[serde(rename(serialize = "d", deserialize = "d"))]
    Debit,
    #[serde(rename(serialize = "c", deserialize = "c"))]
    Credit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTransactionRequest {
    #[serde(rename(deserialize = "valor"))]
    value: u8,
    #[serde(rename(deserialize = "tipo"))]
    tr_type: TransactionType,
    #[serde(rename(deserialize = "descricao"))]
    description: String,
    #[serde(skip_deserializing)]
    client_id: u8,
}

#[derive(Serialize)]
pub struct Balance {
    total: i32,
    #[serde(rename(serialize = "data_extrato"))]
    date: String,
    #[serde(rename(serialize = "limite"))]
    limit: i32,
}

#[derive(Serialize, Debug)]
pub struct Transaction {
    #[serde(rename(serialize = "valor"))]
    value: u8,
    #[serde(rename(serialize = "tipo"))]
    tr_type: TransactionType,
    #[serde(rename(serialize = "descricao"))]
    description: String,
    #[serde(rename(serialize = "criado_em"))]
    created_at: String,
}

#[derive(Serialize)]
pub struct Statement {
    #[serde(rename(serialize = "saldo"))]
    balance: Balance,
    #[serde(rename(serialize = "ultimas_transacoes"))]
    transactions: Vec<Transaction>,
}
