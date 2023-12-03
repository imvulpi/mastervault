use std::{fs::{self, File}, io::{BufReader, BufRead}};

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
            Err(_) => {return true},
        }
    }
}