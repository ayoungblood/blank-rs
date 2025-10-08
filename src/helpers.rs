// debug function name
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}

// helper macro for inline conditional coloring
#[macro_export]
macro_rules! color {
    ($text:expr, $color:ident) => {
        format!("{}", $text.if_supports_color(Stream::Stdout, |text| format!("{}", text.$color())))
    };
    ($text:expr, $color:ident.$style:ident) => {
        format!("{}", $text.if_supports_color(Stream::Stdout, |text| format!("{}", text.$color().$style())))
    };
    ($text:expr, $style:ident.$color:ident) => {
        format!("{}", $text.if_supports_color(Stream::Stdout, |text| format!("{}", text.$style().$color())))
    };
}
