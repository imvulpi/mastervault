use std::{fs::File, io::{LineWriter, Write, Error}};

pub fn create_file_key_value(data : Vec<(String, String)>, path: &str) -> Result<(), Error>{
    match File::options().write(true).create(true).open(path){
        Ok(file) => {
            let mut writer = LineWriter::new(file);
            for (key, data) in data{
                let line = format!("{}={}\n",key, data);
                match writer.write_all(&line.into_bytes()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
            }
        },
        Err(e) => {return Err(e);}
    };
    return Ok(());
}