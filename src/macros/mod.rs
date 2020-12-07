macro_rules! log {
    ($($arg:tt)*) => (
        if crate::OPTS.verbose {
            println!("{} {}!", "[ LOG ]".yellow().bold(), format_args!($($arg)*))
        }
    )
}
