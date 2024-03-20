use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct PizzaRequest {
    #[validate(length(min = 1, message = "A Name is required for the pizza!"))]
    pub name: String,
}
