use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResults<T> {
    pub results: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
}
