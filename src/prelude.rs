#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_print!($($arg)*);
        $crate::serial_print!($($arg)*);
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

