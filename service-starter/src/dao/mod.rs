#[macro_use]
pub mod base;

pub mod admin;
pub mod builder;

#[macro_export]
macro_rules! get_db_str_field {
    ($field_name:expr) => {
        String::from_value_opt($field_name).unwrap_or("".to_string())
    };
}

#[macro_export]
macro_rules! get_db_number_field {
    ($field_type:tt, $field_name:expr, $def_value: expr) => {
        $field_type::from_value_opt($field_name).unwrap_or($def_value)
    };
}
