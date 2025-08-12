use core::{fmt::Write, str};

pub struct FormattedValue {
    data: *const u8,
    len: usize,
}

impl FormattedValue {
    pub fn new() -> Self {
        Self {
            data: core::ptr::null(),
            len: 0,
        }
    }
    pub fn get(&self) -> &'static str {
        unsafe { str::from_raw_parts(self.data, self.len) }
    }
}

impl Write for FormattedValue {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.data = s.as_ptr();
        self.len = s.len();

        Ok(())
    }
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut val = $crate::format::FormattedValue::new();

        write!(val, "{}", format_args!($($arg)*)).expect("Cannot format args");

        val.get()
    }};
}
