#[macro_export]
macro_rules! format {
    ($n: expr, $fmt:expr) => {{
        use core::fmt::Write;

        let mut writer = string_writer::StringWriter::< $n >::new();
        writer.write_fmt(format_args!($fmt)).unwrap();
        unsafe { core::mem::transmute::<&str, &'static str>(writer.as_str().unwrap()) }
    }};
    ($n: expr, $fmt:expr, $($args:tt)*) => {{
        use core::fmt::Write;

        let mut writer = string_writer::StringWriter::< $n >::new();
        writer.write_fmt(format_args!($fmt, $($args)*)).unwrap();
        unsafe { core::mem::transmute::<&str, &'static str>(writer.as_str().unwrap()) }
    }};
}
