#[rust_sitter::grammar("widgets")]
pub mod grammar {
    use crate::widgets::WidgetType;

    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum Expr {
        Window(
            #[rust_sitter::leaf(text = "{")] (),
            #[rust_sitter::leaf(text = "Window:")] (),
            WindowTitle,
            Widget,
            #[rust_sitter::leaf(text = "}")] (),
        ),
    }

    #[derive(Debug)]
    pub struct Widget {
        #[rust_sitter::leaf(text = "{")]
        _param: (),
        #[rust_sitter::leaf(text = "Widget:")]
        _widget: (),
        #[rust_sitter::leaf(pattern = r"\w+", transform = |w| w.to_string().into())]
        pub kind: WidgetType,
        pub class: Option<Class>,
        #[rust_sitter::delimited(
        #[rust_sitter::leaf(text = ";")] ())]
        pub widgets: Vec<Widget>,
        #[rust_sitter::leaf(text = "}")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct Class {
        #[rust_sitter::leaf(text = r"$class")]
        _class: (),
        #[rust_sitter::leaf(text = "\"")]
        _param: (),
        #[rust_sitter::delimited(
            #[rust_sitter::leaf(text = ",")]
            ()
        )]
        pub classes: Vec<ClassName>,
        #[rust_sitter::leaf(text = "\"")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct WindowTitle {
        #[rust_sitter::leaf(text = "\"")]
        _param: (),
        #[rust_sitter::leaf(pattern = r"[\w\s]+", transform = |v| v.to_string())]
        pub title: String,
        #[rust_sitter::leaf(text = "\"")]
        _param_end: (),
    }

    impl From<WindowTitle> for String {
        fn from(window: WindowTitle) -> Self {
            window.title
        }
    }

    #[derive(Debug)]
    pub struct ClassName {
        #[rust_sitter::leaf(pattern = r"\w+", transform = |v| v.to_string())]
        pub name: String,
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s|%%.*%%|\t")]
        _whitespace: (),
    }
}
