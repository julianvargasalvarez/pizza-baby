use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PizzaOrder {
    pub id: i32,
    pub size: String,
    pub quantity: i32,
}
