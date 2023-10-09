use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub fn read_file(args: &mut [String], flag: &str) -> Result<String, String> {
    let mut args = args.iter();
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

pub fn help_args(args: &mut [String]) -> Option<()> {
    if args.iter().any(|x| x == "-h") {
        print_help();
        Some(())
    } else {
        None
    }
}

pub fn print_help() -> &'static str {
    "Thymine\n\nthymine -f [filepath] [options]\n\n-h\tPrints this message\n-f\tFilepath\n-c\tCss filepath"
}
