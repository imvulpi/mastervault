use std::collections::HashSet;

use crate::commons::{fields::traits::{validatable::Validatable, validation_error::ValidationError}, fields::structs::errors::email::EmailValidationErrors};
pub struct EmailData {
    pub email : String,
} 

impl Validatable for EmailData{

    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        if self.email.contains("@") && self.email.contains("."){
            let email_chars : Vec<char> = self.email.chars().collect();

            if email_chars[0] == '.' || email_chars[email_chars.len()-1] == '.'{
                return (false, Some(&EmailValidationErrors::LastOrFirstIsDot));
            }

            let mut at_sign_number = 0;
            let mut previous_char = ' ';
            let alphabet_numbers_set = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',]);
            let special_character_set = HashSet::from(['.', '_', '-',]);

            for &character in email_chars.iter(){
                if character == '.' ||  character == '@'{
                    if character == '@' {
                        at_sign_number += 1;
                        if at_sign_number >= 2{
                            return (false, Some(&EmailValidationErrors::MultipleAtSigns));
                        }
                    }

                    match previous_char {
                        '@' => {
                            if character == '@'{
                                return (false, Some(&EmailValidationErrors::MultipleAtSigns));
                            }else{
                                return (false, Some(&EmailValidationErrors::AtSignNextToDot));
                            }
                        },
                        '.' => {return (false, Some(&EmailValidationErrors::ConsecutiveDots));}
                        _ => {}
                    }
                }

                let low_char = character.to_lowercase().next().unwrap();
                let low_previous_char = previous_char.to_lowercase().next().unwrap();

                if special_character_set.contains(&low_previous_char) && special_character_set.contains(&low_char){
                    if previous_char == character{
                        return (false, Some(&EmailValidationErrors::ConsecutiveCharacters));
                    }
                }

                if !alphabet_numbers_set.contains(&low_char) && !special_character_set.contains(&low_char) && low_char != '@'{
                    return (false, Some(&EmailValidationErrors::NotAllowedCharacter));
                }    

                previous_char = character;
            }

            return (true, None);
        }else{
            if self.email.contains("@"){
                return (false, Some(&EmailValidationErrors::AtSignMissing));
            }
            return (false,Some(&EmailValidationErrors::DomainElementMissing));
        }
    }

    fn put_value(&mut self, value: String) {
        self.email = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.email
    }

    fn new(value: String) -> Self{
        EmailData { email: value }
    }

    fn cclone(&self) -> Box<dyn Validatable>  {
        let clone = EmailData { email: self.email.clone() };
        Box::new(clone)    
    }
}