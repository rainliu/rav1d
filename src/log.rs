#![macro_use]

#[cfg(feature="debug_log")]
macro_rules! rav1d_log {
    ($($arg:tt)*) => (print!($($arg)*));
}

#[cfg(not(feature="debug_log"))]
macro_rules! rav1d_log {
    ($($arg:tt)*) => {};
}