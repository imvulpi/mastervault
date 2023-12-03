use crate::commons::{fields::traits::{validatable::Validatable, validation_error::ValidationError}, fields::structs::errors::parallelism::ParallelismValidationErrors};

use super::errors::general::GeneralValidationErrors;
pub struct ParallelismData {
    pub parallelism : String,
} 

impl Validatable for ParallelismData{
    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        match self.parallelism.parse::<u32>(){
            Ok(val) =>{
                if val >= 1 && val <= 99999999 {
                    return (true, None);
                }else{
                    return (false, Some(&ParallelismValidationErrors::TooSmall));
                }
            }
            Err(_) => {return (false, Some(&GeneralValidationErrors::U32ConversionFailed))}
        }
    }

    fn put_value(&mut self, value: String) {
        self.parallelism = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.parallelism
    }

    fn new(value: String) -> Self {
        ParallelismData{parallelism: value}
    }

    fn cclone(&self) -> Box<dyn Validatable> {
        let clone = ParallelismData { parallelism: self.parallelism.clone() };
        Box::new(clone)    
    }
}