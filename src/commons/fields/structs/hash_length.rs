use crate::commons::{fields::traits::{validatable::Validatable, validation_error::ValidationError}, fields::structs::errors::hash_length::HashLengthValidationErrors};

use super::errors::general::GeneralValidationErrors;
pub struct HashLengthData {
    pub hash_length : String,
} 

impl Validatable for HashLengthData{
    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        match self.hash_length.parse::<u32>(){
            Ok(val) =>{
                if val >= 4 {
                    return (true, None);
                }else{
                    return (false, Some(&HashLengthValidationErrors::TooSmall));
                }
            }
            Err(_) => {return (false, Some(&GeneralValidationErrors::U32ConversionFailed))}
        }
    }

    fn put_value(&mut self, value: String) {
        self.hash_length = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.hash_length
    }

    fn new(value: String) -> Self{
        HashLengthData {hash_length: value}
    }

    fn cclone(&self) -> Box<dyn Validatable>  {
        let clone = HashLengthData { hash_length: self.hash_length.clone() };
        Box::new(clone)    
    }
}