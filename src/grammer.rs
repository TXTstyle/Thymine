#[rust_sitter::grammar("widgets")]
pub mod grammar {

    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum Expr {
        Window(
            #[rust_sitter::leaf(text = "{")] (),
            #[rust_sitter::leaf(text = "Window:")] (),
            WindowTitle,
            Option<TSWidgetType>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
    }

    #[derive(Debug)]
    pub struct WindowTitle {
        #[rust_sitter::leaf(text = r"$title")]
        class: (),
        pub title: Title,
    }

    #[derive(Debug)]
    pub enum TSWidgetType {
        Button(TSButton),
        Label(TSLabel),
        Box(TSBox),
    }

    #[derive(Debug)]
    pub struct Children {
        #[rust_sitter::repeat(non_empty = true)]
        #[rust_sitter::delimited(
        #[rust_sitter::leaf(text = ";")]
        ()
        )]
        pub children: Vec<TSWidgetType>,
    }

    #[derive(Debug)]
    pub struct TSButton {
        #[rust_sitter::leaf(text = "{")]
        _param: (),
        #[rust_sitter::leaf(text = "Button:")]
        _widget: (),
        pub label: Title,
        pub on_click: OnClick,
        pub class: Option<Class>,
        pub children: Option<Children>,
        #[rust_sitter::leaf(text = "}")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct TSLabel {
        #[rust_sitter::leaf(text = "{")]
        _param: (),
        #[rust_sitter::leaf(text = "Label:")]
        _widget: (),
        pub label: Title,
        pub wrap: Option<Wrap>,
        pub class: Option<Class>,
        pub children: Option<Children>,
        #[rust_sitter::leaf(text = "}")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct TSBox {
        #[rust_sitter::leaf(text = "{")]
        _param: (),
        #[rust_sitter::leaf(text = "Box:")]
        _widget: (),
        pub orientaion: TSOrientaionType,
        pub spacing: Option<Spacing>,
        pub class: Option<Class>,
        pub children: Option<Children>,
        #[rust_sitter::leaf(text = "}")]
        _param_end: (),
    }

    #[derive(Debug, Default)]
    #[non_exhaustive]
    pub struct Spacing {
        #[rust_sitter::leaf(text = r"$spacing")]
        class: (),
        #[rust_sitter::leaf(pattern = r"\d+", transform = |d| d.parse().unwrap())]
        pub spacing: i32,
    }

    #[derive(Debug)]
    #[non_exhaustive]
    pub struct Wrap {
        #[rust_sitter::leaf(text = r"$wrap")]
        class: (),
        pub wrap: Boolean,
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
    pub struct OnClick {
        #[rust_sitter::leaf(text = r"$onClick")]
        _class: (),
        #[rust_sitter::leaf(text = "\"")]
        _param: (),
        #[rust_sitter::leaf(pattern = r"[\w\s'\-]+", transform = |v| v.to_string())]
        pub event: String,
        #[rust_sitter::leaf(text = "\"")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct Title {
        #[rust_sitter::leaf(text = "\"")]
        _param: (),
        #[rust_sitter::leaf(pattern = r"[\w\s!]+", transform = |v| v.to_string())]
        pub title: String,
        #[rust_sitter::leaf(text = "\"")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct TSOrientaionType {
        #[rust_sitter::leaf(text = "\"")]
        _param: (),
        #[rust_sitter::leaf(pattern = r"h|v", transform = |v| v.to_string())]
        pub orientaion: String,
        #[rust_sitter::leaf(text = "\"")]
        _param_end: (),
    }

    #[derive(Debug)]
    pub struct Boolean {
        #[rust_sitter::leaf(pattern = r"true|false", transform = |v| v.parse().unwrap())]
        pub boolean: bool,
    }

    #[derive(Debug)]
    pub struct ClassName {
        #[rust_sitter::leaf(pattern = r"\w+", transform = |v| v.to_string())]
        pub name: String,
    }

    impl From<Title> for String {
        fn from(window: Title) -> Self {
            window.title
        }
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s|%%.*%%|%%%.*|\t")]
        _whitespace: (),
    }
}
