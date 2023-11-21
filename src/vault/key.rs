use core::panic;
use std::{io::{self, LineWriter, Write, BufRead, BufReader}, fs::File};
use argon2::{Argon2, password_hash::SaltString, password_hash::{rand_core::OsRng, Ident}, PasswordHasher, Params};
use chacha20poly1305;


pub fn create_master_password(){
    let mut data: Vec<String> = Vec::new();
    let mut input = String::new();
    println!("Please enter an master password: ");
    match io::stdin().read_line(&mut input){
        Ok(_) => {},
        Err(e) => {
            println!("Entered master password caused an error: {}", e);
            panic!("{}", e);
        }
    }

    //  Check for complexity of input.

    match generate_hash_password(input) {
        Ok((password, salt)) => {
            println!("Salt: {} Hash: {}", salt, password);         
            data.push(password);
            data.push(salt);   
        }

        Err(err) => {
            let mut option: String = String::new();
            println!("Error occured: {}\nTry again? Y | N", err);
            match io::stdin().read_line(&mut option){
                Ok(_) => {},
                Err(e) => {
                    panic!("{}", e);
                }
            }

            if option.contains("y") {
                create_master_password();
            }
        }
    }


    write_to_file("key.txt", data);
}

fn write_to_file(path: &str, content: Vec<String>) {
    match File::options().create_new(true).write(true).open(path){
        Ok(result) => {
            let mut writer = LineWriter::new(result);
            for mut argument in content {
                argument += "\n";
                match writer.write(&argument.as_bytes()){
                    Ok(_) => {},
                    Err(_err) =>{return;}
                }

            }  
        }
        Err(_err) => {
            return;
        }
    };
}

fn read_from_file(path: &str) -> Result<Vec<String>, String>{
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
                        return Err(format!("An error occurred: {}", err));
                    }
                }
            }
        }
        Err(err) => {
            return Err(format!("An error occurred: {}", err));
        }
    }

    return Ok(file_content);
}

fn generate_hash_password(password: String) -> Result<(String, String), String>{

    let params : Params;
    match Params::new(32768,6, 1, std::option::Option::Some(32)) {
        Ok(p) => {
            params = p;
        },
        Err(e) => {
            return Err(e.to_string())
        },
    }

    let salt = SaltString::generate(&mut OsRng);

    let algo_id = Option::Some(Ident::new_unwrap("argon2id"));
    let version = Option::Some(19);

    let password_hasher = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    match password_hasher.hash_password_customized(password.as_bytes(), algo_id, version, password_hasher.params().clone(), salt.as_salt()){
        Ok(uncompleted_hash) => {
            let mut hash = String::new();
            match uncompleted_hash.hash.ok_or("Error occurred: Couldn't unwrap the hash!"){
                Ok(o) =>{
                    hash = o.to_string();
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
            return Ok((hash, uncompleted_hash.salt.unwrap().to_string()));
        }
        Err(e) => {return Err(e.to_string());}
    }
}