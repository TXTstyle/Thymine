use std::process::{Command, exit};
use std::rc::Rc;

use crate::widgets::*;
use gtk::glib::clone;
use gtk::{prelude::*, CssProvider, gdk};
use gtk::{Application, ApplicationWindow};

fn build_button(button: &Button) -> gtk::Button {
    let gtk_button = gtk::Button::builder();
    let gtk_button = gtk_button.label(&button.label);

    let gtk_button = gtk_button.build();

    if let Some(class) = &button.class {
        // Apply classes to the button
        for class_name in class.iter() {
            gtk_button.add_css_class(class_name);
        }
    }

    let on_click = &button.on_click;
    gtk_button.connect_clicked(clone!(@strong on_click => move |_| {
        // Handle button click
        // println!("Button clicked: {}", on_click);
        let mut args = on_click.split_whitespace();
        let cmd = Command::new(args.next().unwrap_or_else(|| panic!("Error; button on_click event failed; {}", on_click)))
        .args(args)
        .spawn();

        match cmd {
            Ok(_) => {},
            Err(err) => {
            eprintln!("Error; Unable to call button event `{}`, {}", on_click, err);
            exit(1)
            },
        }
    
    
    }));
    gtk_button
}

fn build_label(label: &Label) -> gtk::Label {
    let gtk_label = gtk::Label::builder();
    let gtk_label = gtk_label.label(&label.label);
    let gtk_label = gtk_label.wrap(label.wrap);

    let gtk_label = gtk_label.build();

    if let Some(class) = &label.class {
        // Apply classes to the label
        for class_name in class.iter() {
            gtk_label.add_css_class(class_name);
        }
    };
    gtk_label
}

fn build_box(box_widget: &BoxWidget) -> gtk::Box {
    let gtk_box = gtk::Box::builder();

    let orientation = match box_widget.orientation {
        Orientation::Vertical => gtk::Orientation::Vertical,
        Orientation::Horizontal => gtk::Orientation::Horizontal,
    };

    let gtk_box = gtk_box.orientation(orientation);
    let gtk_box = gtk_box.spacing(box_widget.spacing);

    let gtk_box = gtk_box.build();

    if let Some(class) = &box_widget.class {
        // Apply classes to the box
        for class_name in class.iter() {
            gtk_box.add_css_class(class_name);
        }
    }

    if let Some(children) = &box_widget.children {
        for child_widget in children {
            let gtk_child = build_widget(child_widget);
            gtk_box.append(&gtk_child);
        }
    }
    gtk_box
}

fn build_widget(widget: &WidgetType) -> gtk::Widget {
    match widget {
        WidgetType::Button(button) => gtk::Widget::from(build_button(button)),
        WidgetType::Label(label) => gtk::Widget::from(build_label(label)),
        WidgetType::Box(box_widget) => gtk::Widget::from(build_box(box_widget)),
    }
}

fn build_window(app: &Application, window: &Window) -> Option<ApplicationWindow> {
    let gtk_window = ApplicationWindow::builder();
    let gtk_window = gtk_window.title(&window.title);
    let gtk_window = gtk_window.application(app);

    if let Some(child_widget) = &window.child {
        let gtk_child = build_widget(child_widget);
        let gtk_window = gtk_window.child(&gtk_child);
        Some(gtk_window.build())
    } else {
        None
    }
}

pub fn build_ui(app: &Application, data: Rc<Window>) {
    let window = match build_window(app, &data) {
        Some(w) => w,
        None => {
            panic!("hello")
        }
    };

    window.present();
}

pub fn load_css(css: String) {
    let provider = CssProvider::new();
    provider.load_from_string(&css);

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("No display found."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
