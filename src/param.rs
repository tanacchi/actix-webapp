use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ParamsForRegister {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsForNewCategory {
    pub name: String,
}
