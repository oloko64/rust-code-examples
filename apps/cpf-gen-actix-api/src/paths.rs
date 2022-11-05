use actix_web::{get, web, Responder};

use crate::types::{
    GenCpfInfo, ResponseErrorCustom, ValidCpfResponse, ValidateCpf, ValidateResponse,
};
use crate::utils;

#[get("/gen-cpf")]
pub async fn new_cpf(
    query_params: web::Query<GenCpfInfo>,
) -> Result<impl Responder, ResponseErrorCustom> {
    let mut qtd = match query_params.qtd {
        Some(ref qtd) => match qtd.parse::<u32>() {
            Ok(qtd) => qtd,
            Err(_) => {
                return Err(ResponseErrorCustom {
                    message: "Invalid \"qtd\". Must be a number between 1 and 1000.",
                })
            }
        },
        None => 1,
    };
    let state_code: Option<u8> = match &query_params.state_code {
        Some(state_code) => match utils::parse_state_code(state_code) {
            Ok(state_code) => Some(state_code),
            Err(_) => {
                return Err(ResponseErrorCustom {
                    message: "Invalid state code. Must be a number between 0 and 9.",
                })
            }
        },
        None => None,
    };
    if qtd > 1000 {
        qtd = 1000
    }
    let mut cpfs = Vec::new();
    (0..qtd)
        .into_iter()
        .for_each(|_| cpfs.push(utils::generate_cpf(state_code, None)));
    Ok(web::Json({
        ValidCpfResponse {
            cpfs,
            message: format!("{} CPFs generated.", qtd),
            quantity: qtd,
        }
    }))
}

#[get("/validate-cpf")]
pub async fn validate_cpf(query: web::Query<ValidateCpf>) -> impl Responder {
    let cpf = match &query.cpf {
        Some(cpf) => cpf,
        None => return web::Json(ValidateResponse { is_valid: false, error: Some("CPF not provided. Inform the cpf in the query params: '/validate-cpf?cpf=123456789012'".to_string()) }),
    };
    let is_valid = utils::validate_cpf(cpf);
    if let Err(error_message) = is_valid {
        web::Json(ValidateResponse {
            is_valid: false,
            error: Some(error_message.to_string()),
        })
    } else {
        web::Json(ValidateResponse {
            is_valid: true,
            error: None,
        })
    }
}
