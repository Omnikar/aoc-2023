pub type Part = fn(&str) -> Box<dyn std::fmt::Display>;

#[macro_export]
macro_rules! box_fun {
    ($fun:expr) => {{
        fn __box_fun(input: &str) -> ::std::boxed::Box<dyn ::std::fmt::Display> {
            ::std::boxed::Box::new($fun(input))
        }
        __box_fun
    }};
}

#[macro_export]
macro_rules! parts {
    ($($part:ident)*) => {
        ::lazy_static::lazy_static! {
            pub static ref PARTS: ::std::collections::HashMap<
                &'static str,
                $crate::macros::Part,
            > = [
                $((stringify!($part), $crate::box_fun!($part) as $crate::macros::Part),)*
            ]
            .into_iter()
            .collect();
        }
    };
}

#[macro_export]
macro_rules! days {
    ($($day:ident)*) => {
        ::lazy_static::lazy_static! {
            pub static ref DAYS: ::std::collections::HashMap<
                &'static str,
                &'static ::std::collections::HashMap<&'static str, $crate::macros::Part>,
            > = [
                $((stringify!($day), &*$day::PARTS),)*
            ]
            .into_iter()
            .collect();
        }
    };
}
