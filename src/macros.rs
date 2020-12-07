macro_rules! log {
    ($($arg:tt)*) => (
        if !crate::OPTS.quiet {
            println!("{} {}!", "[ LOG ]".yellow().bold(), format_args!($($arg)*))
        }
    )
}
