macro_rules! surql {
    ( $( $name:ident $(,)* )+ ) => {
        paste::item! {
            $(
                pub static [<$name:upper>] : &str = include_str!(
                    stringify!([<$name:lower>].surql)
                );
            )+
        }
    };
}

surql! {
    build,
    update,
    system_create,
    system_get_by_name,
    file_create,
}
