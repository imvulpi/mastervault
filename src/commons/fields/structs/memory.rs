use crate::commons::{fields::traits::{validatable::Validatable, validation_error::ValidationError}, fields::structs::errors::memory::MemoryValidationErrors};

use super::errors::general::GeneralValidationErrors;
pub struct MemoryData {
    pub memory_in_kib : String,
} 

impl Validatable for MemoryData{
    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        match self.memory_in_kib.parse::<u32>(){
            Ok(val) =>{
                if val >= 16384 {
                    return (true, None);
                }else{
                    return (false, Some(&MemoryValidationErrors::TooSmall));
                }
            }
            Err(_) => {return (false, Some(&GeneralValidationErrors::U32ConversionFailed))}
        }
    }

    fn put_value(&mut self, value: String) {
        self.memory_in_kib = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.memory_in_kib
    }

    fn new(value: String) -> Self {
        MemoryData {memory_in_kib: value}       
    }

    fn cclone(&self) -> Box<dyn Validatable>  {
        let clone = MemoryData { memory_in_kib: self.memory_in_kib.clone() };
        Box::new(clone)    
    }
}