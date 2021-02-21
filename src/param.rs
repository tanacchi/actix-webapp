use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ParamsForSignUp {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsForSignIn {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsForNewCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsForNewReport {
    pub date: String,
    pub comment: String,
    pub category: i64,
}
