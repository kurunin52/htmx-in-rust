use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct OrderBalance {
    pub order_id: i32,
    pub order_status: String,
}

pub type OrderError = String;
pub type OrderResult<T> = Result<T, OrderError>;

#[async_trait::async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn get_order_balance_list(&self) -> OrderResult<Vec<OrderBalance>>;
    async fn search_order_balance(&self, status: &str) -> OrderResult<Vec<OrderBalance>>;
}

pub async fn search_order_balance(
    order_repository: &impl OrderRepository,
    status: &str,
) -> OrderResult<Vec<OrderBalance>> {
    order_repository.search_order_balance(status).await
}

pub async fn get_order_balance_list(
    order_repository: &impl OrderRepository,
) -> OrderResult<Vec<OrderBalance>> {
    order_repository.get_order_balance_list().await
}
