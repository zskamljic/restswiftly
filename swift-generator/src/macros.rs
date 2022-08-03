#[macro_export]
macro_rules! write_indent {
    ($dst:expr, $indent:expr, $($arg:tt)*) => {
        {
            $dst.write_all(" ".repeat($indent as usize).as_bytes())?;
            write!($dst, $($arg)*)
        }
    };
}

#[macro_export]
macro_rules! writeln_indent {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $indent:expr, $($arg:tt)*) => {
        {
            $dst.write_all(" ".repeat($indent as usize).as_bytes())?;
            writeln!($dst, $($arg)*)
        }
    };
}
