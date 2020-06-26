use std::env;
use std::fs;
use std::io::{Write, stdout};
use colored::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    read_dir(args[1].to_owned());
}

fn read_dir(_path: String){
    let paths = fs::read_dir(_path).unwrap();
    for path in paths{
        let path = path.unwrap().path();
        if path.is_dir(){
            print!("{}\\    ", path.file_name().unwrap().to_owned().into_string().unwrap())
        }else{
            print!("{}  ", path.file_name().unwrap().to_owned().into_string().unwrap())
        }
        
    }
    stdout().flush().unwrap();
}
