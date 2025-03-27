use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct ListDto<T> {
    items: Vec<T>,
    total_items: i64,
    page: usize,
    page_size: usize,
    has_next: bool,
    has_prev: bool,
}

impl<T> ListDto<T> {
    pub fn new(items: Vec<T>, total_items: i64, page: usize, page_size: usize) -> Self {
        let has_next = page * page_size < total_items as usize;
        let has_prev = page > 1;
        ListDto {
            items,
            total_items,
            page,
            page_size,
            has_next,
            has_prev,
        }
    }
}
