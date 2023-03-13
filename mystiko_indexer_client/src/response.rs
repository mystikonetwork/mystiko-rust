use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub result: T,
}
