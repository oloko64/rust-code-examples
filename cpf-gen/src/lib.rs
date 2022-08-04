use rand::Rng;

#[derive(Debug)]
pub struct Cpf {
    pub cpf: String,
    pub cpf_formatted: String,
    pub cpf_state: Vec<String>,
}

pub fn generate_cpf(state_code: Option<u8>) -> Cpf {
    let cpf_seed = match state_code {
        Some(state_code) => {
            let mut seed = random_seed();
            seed[8] = state_code;
            seed
        }
        None => random_seed(),
    };
    let sum_elements:u32 = cpf_seed.iter().enumerate().map(|(index, &number)| number as u32 * (10 - index as u32)).sum::<u32>();
    let first_verifier_num = verifier_num(sum_elements);
    let mut cpf_seed_with_first_verifier: Vec<u8> = cpf_seed.clone().into_iter().chain(vec![first_verifier_num]).collect::<Vec<u8>>();
    cpf_seed_with_first_verifier.remove(0);
    let sum_elements_with_first_verifier = cpf_seed_with_first_verifier.iter().enumerate().map(|(index, &number)| number as u32 * (10 - index as u32)).sum::<u32>();
    let second_verifier_num = verifier_num(sum_elements_with_first_verifier);
    let cpf: String = cpf_seed.into_iter().chain(vec![first_verifier_num, second_verifier_num]).collect::<Vec<u8>>().iter().map(|&number| number.to_string()).collect::<Vec<String>>().join("");
    let formatted_cpf = format!("{}.{}.{}-{}", cpf[0..3].to_string(), cpf[3..6].to_string(), cpf[6..9].to_string(), cpf[9..11].to_string());
    Cpf {
        cpf: cpf.clone(),
        cpf_formatted: formatted_cpf,
        cpf_state: cpf_state(&cpf).iter().map(|&state| state.to_string()).collect::<Vec<String>>(),
    }
}

fn verifier_num(n1: u32) -> u8 {
    let n2 = n1 % 11;
    if n2 < 2 {  0 } else { (11 - n2).try_into().unwrap() }
}

fn random_seed() -> [u8; 9] {
    let mut seed = [0; 9];
    for i in 0..9 {
        seed[i] = rand::thread_rng().gen_range(0..10);
    }
    seed
}

fn cpf_state(cpf: &str) -> Vec<&str> {
    let state_code: char = cpf.chars().nth(8).unwrap();
    match state_code {
        '0' => vec!["RS"],
        '1' => vec!["DF", "GO", "MT", "MS", "TO"],
        '2' => vec!["AC", "AM", "AP", "PA", "RO", "RR"],
        '3' => vec!["CE", "MA", "PI"],
        '4' => vec!["AL", "PB", "PE", "RN"],
        '5' => vec!["BA", "SE"],
        '6' => vec!["MG"],
        '7' => vec!["ES", "RJ"],
        '8' => vec!["SP"],
        '9' => vec!["PR", "SC"],
        _ => panic!("Invalid state code"),
    }
}
