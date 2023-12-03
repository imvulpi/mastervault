use crate::commons::{fields::traits::{validation_error::ValidationError, self, validatable::Validatable}, fields::structs::errors::algorithm::AlgorithmValidationErrors};
pub struct AlgorithmData {
    pub algorithm : String,
} 

impl traits::validatable::Validatable for AlgorithmData{
    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        let algorithms = vec!["argon2", "argon2i","argon2id"];
        for algorithm in algorithms{
            if self.algorithm == algorithm{
                return (true, None);
            }
        }
        return (false, Some(&AlgorithmValidationErrors::NotFound));
    }

    fn put_value(&mut self, value: String) {
        self.algorithm = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.algorithm
    }

    fn new(value: String) -> Self{
        AlgorithmData { algorithm: value }
    }

    fn cclone(&self) -> Box<dyn Validatable> {
        let clone = AlgorithmData { algorithm: self.algorithm.clone() };
        Box::new(clone)
    }
}