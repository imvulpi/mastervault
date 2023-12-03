use std::ops::Index;
use crate::constants::config::defaults::get_config_template;

pub fn remove_invalid_tuples(data: &mut Vec<(String, String)>){
    let mut config_template = get_config_template();
    for i in (0..data.len()).rev(){
        let mut is_invalid = true;
        let (setting, option) = data.index(i);
        for (template_setting, template_option) in &mut config_template{
            template_option.put_value(option.to_owned());
            if setting == template_setting{
                if template_option.validate().0 == true{
                    is_invalid = false;
                }else{
                    is_invalid = true;
                }
            }
        }
        if is_invalid {
            data.remove(i);
        }
    }
}

pub fn get_missing_tuples(data: &mut Vec<(String, String)>) -> Vec<&str>{
    let mut missing_settings = Vec::new();
    let config_template = get_config_template();
    
    for i in (0..config_template.len()).rev(){
        let mut is_invalid = true;
        let setting = config_template.index(i).0;
        for (template_setting, _) in &mut *data{
            if setting == template_setting{
                is_invalid = false;
            }
        }
        if is_invalid {
            missing_settings.push(config_template.index(i).0);
        }
    }
    missing_settings
}
