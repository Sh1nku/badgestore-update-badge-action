use std::env;
use std::env::VarError;

pub fn get_input(name: &str) -> Option<String> {
    match env::var(format!("INPUT_{}", name.to_uppercase())) {
        Ok(val) => match val.is_empty() {
            true => None,
            false => match val.as_str() {
                "__NOT_USED__" => None,
                _ => Some(val),
            },
        },
        Err(_) => None,
    }
}

pub fn get_env(var: &str) -> Result<String, VarError> {
    env::var(var)
}
