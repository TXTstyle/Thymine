use crate::grammar::*;

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub child: Option<WidgetType>,
}

#[derive(Debug)]
pub struct Button {
    pub label: String,
    pub on_click: String,
    pub class: Option<Vec<String>>,
    pub children: Option<Vec<WidgetType>>,
}

#[derive(Debug)]
pub struct Label {
    pub label: String,
    pub wrap: bool,
    pub class: Option<Vec<String>>,
    pub children: Option<Vec<WidgetType>>,
}

#[derive(Debug)]
pub struct BoxWidget {
    pub orientation: Orientation,
    pub spacing: i32,
    pub class: Option<Vec<String>>,
    pub children: Option<Vec<WidgetType>>,
}

#[derive(Debug)]
pub enum WidgetType {
    Button(Button),
    Label(Label),
    Box(BoxWidget),
}

#[derive(Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl From<Expr> for Window {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Window(_, _, title, widget, _) => Window {
                title: title.title.title,
                child: widget.map(|w| w.into())
            },
        }
    }
}

impl From<TSWidgetType> for WidgetType {
    fn from(widget: TSWidgetType) -> Self {
        match widget {
            TSWidgetType::Button(b) => WidgetType::Button(b.into()),
            TSWidgetType::Label(l) => WidgetType::Label(l.into()),
            TSWidgetType::Box(b) => WidgetType::Box(b.into()),
        }
    }
}

impl From<WidgetType> for Button {
    fn from(widget: WidgetType) -> Self {
        match widget {
            WidgetType::Button(button) => button,
            _ => panic!("Unexpected WidgetType for Button conversion"),
        }
    }
}

impl From<TSButton> for Button {
    fn from(ts_button: TSButton) -> Self {
        Button {
            label: ts_button.label.title,
            on_click: ts_button.on_click.event,
            class: ts_button.class.map(convert_class),
            children: ts_button.children.map(|children| {
                children
                    .children
                    .into_iter()
                    .map(WidgetType::from)
                    .collect()
            }),
        }
    }
}

impl From<WidgetType> for Label {
    fn from(widget: WidgetType) -> Self {
        match widget {
            WidgetType::Label(label) => label,
            _ => panic!("Unexpected WidgetType for Label conversion"),
        }
    }
}

impl From<TSLabel> for Label {
    fn from(ts_label: TSLabel) -> Self {
        Label {
            label: ts_label.label.title,
            wrap: ts_label.wrap.map_or(false, |wrap| wrap.wrap.boolean),
            class: ts_label.class.map(convert_class),
            children: ts_label.children.map(|children| {
                children
                    .children
                    .into_iter()
                    .map(WidgetType::from)
                    .collect()
            }),
        }
    }
}

impl From<WidgetType> for BoxWidget {
    fn from(widget: WidgetType) -> Self {
        match widget {
            WidgetType::Box(box_widget) => box_widget,
            _ => panic!("Unexpected WidgetType for BoxWidget conversion"),
        }
    }
}

impl From<TSBox> for BoxWidget {
    fn from(ts_box: TSBox) -> Self {
        BoxWidget {
            orientation: ts_box.orientaion.into(),
            spacing: ts_box.spacing.unwrap_or_default().spacing,
            class: ts_box.class.map(convert_class),
            children: ts_box.children.map(|children| {
                children
                    .children
                    .into_iter()
                    .map(WidgetType::from)
                    .collect()
            }),
        }
    }
}

impl From<TSOrientaionType> for Orientation {
    fn from(orientation_type: TSOrientaionType) -> Self {
        match orientation_type.orientaion.as_str() {
            "h" => Orientation::Horizontal,
            "v" => Orientation::Vertical,
            _ => panic!("Unexpected orientation type"),
        }
    }
}

fn convert_class(class: Class) -> Vec<String> {
    class.classes.into_iter().map(|name| name.name).collect()
}
