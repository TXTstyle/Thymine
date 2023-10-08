use crate::grammar;

#[derive(Debug)]
pub struct Window {
    title: String,
    child: Widget,
}

#[derive(Debug)]
pub struct Widget {
    pub kind: WidgetType,
    pub class: Vec<String>,
    pub children: Vec<Widget>,
}

#[derive(Debug)]
pub enum WidgetType {
    Button,
    Label,
}

fn convert_class(class: grammar::Class) -> Vec<String> {
    class.classes.into_iter().map(|name| name.name).collect()
}

impl Window {
    pub fn new(title: String, child: Widget) -> Window {
        Window { title, child }
    }
}

impl From<String> for WidgetType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "button" => WidgetType::Button,
            "label" => WidgetType::Label,
            _ => todo!(),
        }
    }
}

impl From<grammar::Widget> for Widget {
    fn from(widget: grammar::Widget) -> Self {
    let class = widget
        .class
        .map(convert_class)
        .unwrap_or_default();
    let children = widget.widgets.into_iter().map(Widget::from).collect();

    Widget {
        kind: widget.kind,
        class,
        children,
    }
}
}

impl From<grammar::Expr> for Window {
    fn from(expr: grammar::Expr) -> Self {
    match expr {
        grammar::Expr::Window(_, _, title, widget, _) => {
            let child = widget.into();
            Window::new(title.into(), child)
        }
    }
}
}
