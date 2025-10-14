pub mod new;

pub use self::new::handle_new_command;

pub fn handle_error<E>(error: E)
where
    E: std::fmt::Display,
{
    eprintln!("{error}");
    ::std::process::exit(1);
}
