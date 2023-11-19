use std::{fs::{File, self}, io::{BufReader, BufRead, LineWriter, Write}};

pub struct FileChecker{}

impl FileChecker{
    pub fn file_exists(path: &str) -> bool{
        if fs::metadata(path).is_err() {
            return false;
        }
        true
    }

    pub fn is_empty(path: &str) -> bool{
        let file = File::open(path);
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                if reader.lines().count() <= 1 {
                    return true;
                }
                false
            },
            Err(_) => return true,
        }
    }
}

pub fn create_config_file(data : Vec<String>) -> Result<(), String>{
    match File::options().write(true).create(true).open("config.txt"){
        Ok(file) => {
            let mut writer = LineWriter::new(file);
            for option in data{
                print!("{}", option);
                match writer.write_all(&option.into_bytes()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e.to_string());}
                }
            }
        },
        Err(e) => {return Err(e.to_string());}
    };
    return Ok(());
}
