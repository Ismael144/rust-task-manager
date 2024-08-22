#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use validator::Validate;

// I will on validations later
#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(length(min = 5))]
    pub description: String,
    pub is_complete: bool,
}