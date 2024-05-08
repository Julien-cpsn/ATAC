#[macro_export]
macro_rules! print_if_not_in_command {
    ($($x:tt)*) => {
        if !crate::cli::args::ARGS.in_command() {
            println!($($x)*);
        }
    }
}