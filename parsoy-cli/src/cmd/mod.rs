pub mod new;

use core::panic;

pub use new::handle_new_command;

pub fn handle_error<E>(error: E)
where
    E: std::fmt::Display,
{
    eprintln!("{error}");
    ::std::process::exit(1)
}

pub fn handle_panic<E>(error: E)
where
    E: std::string::ToString,
    E: std::error::Error,
{
    panic!("{error}")
}

pub fn format_error<E>(error: E) -> String
where
    E: std::string::ToString,
    E: std::fmt::Display,
    E: std::fmt::Debug
{
    format!("{:#?}", error)
}
