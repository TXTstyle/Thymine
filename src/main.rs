#![allow(dead_code)]
mod widgets;
use widgets::*;
mod grammer;
use grammer::grammar;

use std::{
    env::{args, Args},
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

fn read_file(args: &mut Args) -> Result<String, String> {
    let file_path = match args.position(|x| x == "-f") {
        Some(_) => match args.next() {
            Some(s) => PathBuf::from(s),
            None => {
                return Err("No file path given!".to_string());
            }
        },
        None => {
            return Err("Must give a file flag; -f".to_string());
        }
    };

    match File::open(file_path) {
        Ok(f) => {
            let mut buf = BufReader::new(f);
            let mut res = String::new();
            match buf.read_to_string(&mut res) {
                Ok(_) => Ok(res),
                Err(err) => Err(format!("Unable to read file; {}", err)),
            }
        }
        Err(err) => Err(format!("Unable to open file; {}", err)),
    }
}

fn main() {
    let mut args = args();
    let input = match read_file(&mut args) {
        Ok(i) => i,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let tree = match grammar::parse(&input) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error; {:#?}", err);
            return;
        }
    };
    println!("{:#?}", tree);
    let window = Window::from(tree);
    println!("{:#?}", window)
}
