#![allow(dead_code)]
mod grammer;
mod ui;
mod utils;
mod widgets;
use crate::grammer::to_chic_error;
use chic::Error as ChicError;
use grammer::thymine;
use gtk::prelude::*;
use gtk::{glib, Application};
use std::{env::args, rc::Rc};
use ui::*;
use widgets::*;

const APP_ID: &str = "com.TXTstyle.thymine";

fn main() -> glib::ExitCode {
    let mut args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("{}", utils::print_help());
        return glib::ExitCode::SUCCESS;
    }

    if utils::help_args(&mut args).is_some() {
        println!("{}", utils::print_help());
        return glib::ExitCode::SUCCESS;
    }

    let input = match utils::read_file(&mut args, "-f") {
        Ok(i) => i,
        Err(err) => {
            eprintln!("{}", err);
            return glib::ExitCode::FAILURE;
        }
    };

    let css_data = utils::read_file(&mut args, "-c").ok();
    let mut chic_errors: Vec<ChicError> = Vec::new();

    let tree = match thymine::parse(&input) {
        Ok(s) => s,
        Err(err) => {
            // eprintln!("Error; {:#?}", err);
            let mut errors_tmp = vec![];
            err.into_iter()
                .for_each(|e| errors_tmp.push(to_chic_error(e, input.as_str(), &mut chic_errors)));
            chic_errors.append(&mut errors_tmp);

            for err in chic_errors {
                eprintln!("{}", err.to_string());
            }
            return glib::ExitCode::FAILURE;
        }
    };
    let window = Rc::new(Window::from(tree));

    let app = Application::builder().application_id(APP_ID).build();

    if let Some(css) = css_data {
        app.connect_startup(move |_| load_css(css.to_owned()));
    } else {
        eprintln!("No css file found, not loading css.");
    }
    app.connect_activate(move |a| build_ui(a, window.clone()));
    let empty: Vec<String> = vec![];
    app.run_with_args(&empty)
}
