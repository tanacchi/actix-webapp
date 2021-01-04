use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ParamsForRegister {
    pub name: String,
}
