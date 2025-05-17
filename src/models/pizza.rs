
use serde::{Deserialize, Serialize};
use validator::Validate;





#[derive(Validate, Deserialize, Serialize)]
pub struct Buy_Pizza_Request {
    #[validate(length(min = 1, max = 50, message = "Pizza name must be between 1 and 50 characters"))]
    pub pizza_name: String,
    
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL {
    pub  uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Self {
        Pizza { uuid, pizza_name}
    }
}

