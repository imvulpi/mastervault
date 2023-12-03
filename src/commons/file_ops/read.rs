use std::{fs::File, io::{BufReader, Error, BufRead}};

pub fn get_lines_as_string_vec(path: &str) -> Result<Vec<String>, Error>{
    let mut file_content: Vec<String> = Vec::new();
    match File::options().read(true).open(path){
        Ok(file) =>{
            let reader = BufReader::new(file);
            for line in reader.lines(){
                match line {
                    Ok(line ) => {
                        file_content.push(line);
                    }
                    Err(err) =>{
                        return Err(err);
                    }
                }
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    return Ok(file_content);
}