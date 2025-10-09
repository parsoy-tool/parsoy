pub mod new;

pub fn handle_error<E>(error: E)
where
    E: std::fmt::Display,
{
    eprintln!("{error}");
    ::std::process::exit(1)
}
