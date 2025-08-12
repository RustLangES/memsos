use core::{fmt::Write, str};

pub struct FormatedValue(*const u8, usize);

impl FormatedValue {
    pub fn new() -> Self {
        Self(core::ptr::null(), 0)
    }
    pub fn get(&self) -> &'static str {
        unsafe { str::from_raw_parts(self.0, self.1) }
    }
}

impl Write for FormatedValue {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0 = s.as_ptr();
        self.1 = s.len();

        Ok(())
    }
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut val = $crate::format::FormatedValue::new();

        write!(val, "{}", format_args!($($arg)*)).expect("Cannot format args");

        val
    }};
}
