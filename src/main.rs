#![allow(dead_code)]
mod grammer;
mod utils;
mod widgets;
mod ui;
use grammer::grammar;
use ui::*;
use std::env::args;
use std::rc::Rc;
use widgets::*;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "com.TXTstyle.tree";

fn main() -> glib::ExitCode {
    let mut args = args();
    let input = match utils::read_file(&mut args, "-f") {
        Ok(i) => i,
        Err(err) => {
            eprintln!("{}", err);
            return glib::ExitCode::FAILURE;
        }
    };

    let css_data = utils::read_file(&mut args, "-c").ok();

    let tree = match grammar::parse(&input) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error; {:#?}", err);
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

