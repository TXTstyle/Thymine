use crate::grammer::thymine::*;

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
}

#[derive(Debug)]
pub struct Label {
    pub label: String,
    pub wrap: bool,
    pub class: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct BoxWidget {
    pub orientation: Orientation,
    pub spacing: i32,
    pub class: Option<Vec<String>>,
    pub children: Option<Vec<WidgetType>>,
}

#[derive(Debug)]
pub struct CenterWidget {
    pub orientation: Orientation,
    pub class: Option<Vec<String>>,
    pub child_start: Box<WidgetType>,
    pub child_center: Box<WidgetType>,
    pub child_end: Box<WidgetType>,
}

#[derive(Debug)]
pub enum WidgetType {
    Button(Button),
    Label(Label),
    Box(BoxWidget),
    Center(CenterWidget),
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
                child: widget.map(|w| w.into()),
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
            TSWidgetType::Center(c) => WidgetType::Center(c.into()),
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

impl From<Box<TSWidgetType>> for Box<WidgetType> {
    fn from(widget: Box<TSWidgetType>) -> Self {
        let w = match *widget {
            TSWidgetType::Button(a) => WidgetType::Button(a.into()),
            TSWidgetType::Label(l) => WidgetType::Label(l.into()),
            TSWidgetType::Box(b) => WidgetType::Box(b.into()),
            TSWidgetType::Center(c) => WidgetType::Center(c.into()),
        };
        Box::new(w)
    }
}

impl From<WidgetType> for CenterWidget {
    fn from(widget: WidgetType) -> Self {
        match widget {
            WidgetType::Center(widget) => widget,
            _ => panic!("Unexpected WidgetType for CenterWidget conversion"),
        }
    }
}

impl From<TSCenterBox> for CenterWidget {
    fn from(ts_center: TSCenterBox) -> Self {
        CenterWidget {
            orientation: ts_center.orientaion.into(),
            class: ts_center.class.map(convert_class),
            child_start: ts_center.children.child_1.into(),
            child_center: ts_center.children.child_2.into(),
            child_end: ts_center.children.child_3.into(),
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
