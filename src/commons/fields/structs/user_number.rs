
use crate::commons::fields::traits::validatable::Validatable;
use super::errors::general::GeneralValidationErrors;

pub struct UserNumber {
    number : String,
}

impl Validatable for UserNumber{
    fn validate(&self) -> (bool, Option<&dyn crate::commons::fields::traits::validation_error::ValidationError>) {
        match self.number.parse::<i32>(){
            Ok(_) =>{
                return (true, None);
            }
            Err(_) => {
                return (false, Some(&GeneralValidationErrors::ParsingFailed));
            }
        }
    }

    fn put_value(&mut self, value: String) {
        self.number = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.number
    }

    fn new(value: String) -> Self where Self: Sized {
        UserNumber { number: value }
    }

    fn cclone(&self) -> Box<dyn Validatable>{
        let clone = UserNumber { number: self.number.clone() };
        Box::new(clone)    
    }
}