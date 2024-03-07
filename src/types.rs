use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionType {
    #[serde(rename(serialize = "d", deserialize = "d"))]
    Debit,
    #[serde(rename(serialize = "c", deserialize = "c"))]
    Credit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTransactionRequest {
    #[serde(rename(deserialize = "valor"))]
    pub value: u32,
    #[serde(rename(deserialize = "tipo"))]
    pub tr_type: TransactionType,
    #[serde(rename(deserialize = "descricao"))]
    pub description: String,
    #[serde(skip_deserializing)]
    pub client_id: i8,
}

#[derive(Serialize, Debug)]
pub struct CreateTransactionResponse {
    #[serde(rename(serialize = "limite"))]
    pub limit: i32,
    #[serde(rename(serialize = "saldo"))]
    pub balance: i32,
}

impl CreateTransactionRequest {
    pub fn is_valid(&self) -> bool {
        if self.description.len() > 10 || self.description.is_empty() {
            return false;
        }

        if self.value == 0 {
            return false;
        }

        true
    }
}

#[derive(sqlx::FromRow)]
pub struct Client {
    pub balance: i32,
    pub limit: i32,
    pub id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Balance {
    #[sqlx(rename = "balance")]
    pub total: i32,
    #[serde(rename(serialize = "data_extrato"))]
    pub date: String,
    #[serde(rename(serialize = "limite"))]
    pub limit: i32,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Transaction {
    #[serde(rename(serialize = "valor"))]
    pub value: u32,
    #[serde(rename(serialize = "tipo"))]
    #[sqlx(rename = "type")]
    pub tr_type: TransactionType,
    #[serde(rename(serialize = "descricao"))]
    pub description: String,
    #[serde(rename(serialize = "criado_em"))]
    pub created_at: String,
}

#[derive(Serialize)]
pub struct Statement {
    #[serde(rename(serialize = "saldo"))]
    pub balance: Balance,
    #[serde(rename(serialize = "ultimas_transacoes"))]
    pub transactions: Vec<Transaction>,
}
