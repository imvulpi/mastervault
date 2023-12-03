use crate::commons::fields::traits::validatable::Validatable;

use super::errors::general::GeneralValidationErrors;

pub struct Config {
    pub config : Vec<(&'static str, Box<dyn Validatable>)>
}

impl Config{
    pub fn get_option(&self, setting: &str) -> Result<Box<dyn Validatable>, &GeneralValidationErrors>{
        for (s, option) in &self.clone().config{
            if &setting == s{
                return Ok(option.cclone());
            }
        }
        return Err(&GeneralValidationErrors::SettingNotFound);
    }

    pub fn clone(&self) -> Self{
        let mut cloned : Vec<(&'static str, Box<dyn Validatable>)> = Vec::new();
        for (s, option) in self.config.iter(){
            let option_valid = option.cclone();
            cloned.push((s, option_valid));
        }
        Config { config: cloned }
    }
}