use serde::Deserialize;
use solana_sdk::transaction::TransactionError;

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub id: u64,
    pub jsonrpc: String,
    #[serde(alias = "error")]
    pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct Context {
    pub slot: u64,
}

#[derive(Deserialize, Debug)]
pub struct RpcResultWithContext<T> {
    pub context: Context,
    pub value: T,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockhash {
    pub blockhash: String,
    #[serde(skip)] // TODO latest blockhash
    pub last_valid_block_height: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionError {
    pub code: i64,
    pub data: RpcTransactionErrorData,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionErrorData {
    pub err: TransactionError,
    pub logs: Vec<String>,
}
