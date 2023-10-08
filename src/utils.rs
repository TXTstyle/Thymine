use std::{
    env::Args,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub fn read_file(args: &mut Args, flag: &str) -> Result<String, String> {
    let file_path = match args.position(|x| x == flag) {
        Some(_) => match args.next() {
            Some(s) => PathBuf::from(s),
            None => {
                return Err("No file path given!".to_string());
            }
        },
        None => {
            return Err(format!("Must give a file flag; {}", flag));
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
