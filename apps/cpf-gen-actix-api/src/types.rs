use actix_web::error;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Cpf {
    pub cpf: String,
    pub cpf_formatted: String,
    pub cpf_state: Vec<String>,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "error: {}", message)]
pub struct ResponseErrorCustom {
    pub message: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ResponseErrorCustom {}

#[derive(Deserialize, Debug)]
pub struct GenCpfInfo {
    pub qtd: Option<String>,
    pub state_code: Option<String>,
}

#[derive(Deserialize)]
pub struct ValidateCpf {
    pub cpf: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidCpfResponse {
    pub cpfs: Vec<Cpf>,
    pub message: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub is_valid: bool,
    pub error: Option<String>,
}
