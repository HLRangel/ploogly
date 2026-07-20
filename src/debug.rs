/// Conditional debug printing controlled by feature `quiet`.
/// Default: prints everything. Build with `--features quiet` to disable.

#[cfg(not(feature = "quiet"))]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

#[cfg(feature = "quiet")]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}
