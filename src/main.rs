use grammar::Expr;

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

#[rust_sitter::grammar("arithmetic")]
mod grammar {
    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum Expr {
        Number(
            #[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())]
            u32
        ),
        #[rust_sitter::prec_left(3)]
        Param(
            #[rust_sitter::leaf(text = "(")] (),
            Box<Expr>,
            #[rust_sitter::leaf(text = ")")] (),
        ),
        #[rust_sitter::prec_left(1)]
        Add(
            Box<Expr>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<Expr>
        ),
        #[rust_sitter::prec_left(1)]
        Sub(
            Box<Expr>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expr>
        ),
        #[rust_sitter::prec_left(2)]
        Multi(Box<Expr>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expr>
        ),
        #[rust_sitter::prec_left(2)]
        Div(Box<Expr>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<Expr>
        ),
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}

fn evaluate(expr: &Expr) -> u32 {
    match expr {
        Expr::Number(value) => *value,
        Expr::Add(left, _, right) => evaluate(left) + evaluate(right),
        Expr::Sub(left, _, right) => evaluate(left) - evaluate(right),
        Expr::Multi(left, _, right) => evaluate(left) * evaluate(right),
        Expr::Div(left, _, right) => evaluate(left) / evaluate(right),
        Expr::Param(_, expr, _) => evaluate(expr),
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

    let tree = grammar::parse(&input).unwrap();
    println!("{:#?}", tree);
    let mut _leaves: Vec<u32> = Vec::new();
    let res = evaluate(&tree);
    println!("{:?}", res)
}
