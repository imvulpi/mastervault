use argon2::{Argon2, password_hash::SaltString, password_hash::rand_core::OsRng, PasswordHasher, Params, Algorithm};
use crate::commons::fields::structs::errors::password_hash::PasswordHashError;
pub mod complexity;
pub mod master_key;

pub fn generate_hash_password(password: String, params: Params, algorithm: Algorithm) -> Result<Vec<(String, String)>, PasswordHashError>{
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let version = Option::Some(19);
    let password_hashed = Argon2::new(algorithm, argon2::Version::V0x13, params);
    match password_hashed.hash_password_customized(password.as_bytes(), Some(algorithm.ident()), version, password_hashed.params().clone(), salt.as_salt()){
        Ok(output) => {
            let mut generation_result: Vec<(String, String)> = Vec::new();
            generation_result.push(("password".to_string(), output.hash.unwrap().to_string()));
            generation_result.push(("salt".to_string(), salt.to_string()));
            return Ok(generation_result);
        }
        Err(_) => {
            return Err(PasswordHashError::Other);
        }
    }
}