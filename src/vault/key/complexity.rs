use std::collections::HashSet;
use crate::{commons::fields::structs::errors::password_hash::PasswordHashError, constants::general::values::{MIN_PASSWORD_LENGTH, MIN_PASSWORD_UPPERCASE, MIN_PASSWORD_LOWERCASE, MIN_PASSWORD_SPECIAL}};

pub fn check_password_complexity(password: &String) -> Result<bool, PasswordHashError> {   
    let results = [
        check_length(&password),
        check_uppercase_chars(&password),
        check_lowercase_chars(&password),
        check_special_chars(&password),
    ];
    
    for result in results{
        match result{
            Ok(_) => {},
            Err(error) => {
                return Err(error);
            },
        }
    }

    return Ok(true);
}

pub fn check_length(password: &String) -> Result<bool, PasswordHashError> {
    if password.chars().count() > MIN_PASSWORD_LENGTH as usize {
        return Ok(true);
    }
    Err(PasswordHashError::TooShort)
}

pub fn check_uppercase_chars(password: &String) -> Result<bool, PasswordHashError>{
    let mut uppercase_amount = 0;
    let uppercase_characters = HashSet::from([
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z' 
    ]);

    for char in password.chars(){
        if uppercase_characters.contains(&char){
            uppercase_amount+=1;
        }
    }

    if uppercase_amount >= MIN_PASSWORD_UPPERCASE{
        return Ok(true);
    } 
    return Err(PasswordHashError::NotEnoughUpperCase);
}

pub fn check_lowercase_chars(password: &String) -> Result<bool, PasswordHashError>{
    let mut lowercase_amount = 0;
    let lowercase_characters = HashSet::from([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ]);

    for char in password.chars(){
        if lowercase_characters.contains(&char){
            lowercase_amount+=1;
        }
    }

    if lowercase_amount >= MIN_PASSWORD_LOWERCASE{
        return Ok(true);
    } 
    return Err(PasswordHashError::NotEnoughLowerCase);
}

pub fn check_special_chars(password: &String) -> Result<bool, PasswordHashError>{
    let mut special_amount = 0;
    let special_characters = HashSet::from([
        '!', '@', '#', '$', '%', '^', '&', '*', '_', '-', '+', '=', '?', '.', 
        ',', ';', ':', '|', '~', '`', '<', '>', '{', '}', '[', ']', '(', ')'
    ]);

    for char in password.chars(){
        if special_characters.contains(&char){
            special_amount+=1;
        }
    }

    if special_amount >= MIN_PASSWORD_SPECIAL{
        return Ok(true);
    } 
    return Err(PasswordHashError::NotEnoughSpecialCharacters);
}