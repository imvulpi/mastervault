use crate::commons::{fields::traits::{validatable::Validatable, validation_error::ValidationError}, fields::structs::errors::username::UsernameValidationErrors};
pub struct UsernameData {
    pub username : String,
} 

impl Validatable for UsernameData{
    fn validate(&self) -> (bool, Option<&dyn ValidationError>) {
        let username_length = self.username.chars().count();
        if username_length > 2{
            return (true, None);
        }else{
            return (false, Some(&UsernameValidationErrors::TooShort));
        }
    }

    fn put_value(&mut self, value: String) {
        self.username = value;
    }

    fn get_raw_value(&self) -> &String {
        &self.username
    }
    
    fn new(value: String) -> Self {
        UsernameData{username: value}
    }

    fn cclone(&self) -> Box<dyn Validatable> {
        let clone = UsernameData { username: self.username.clone() };
        Box::new(clone)    
    }
}