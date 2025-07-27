use core::ops::Sub;

use x86_64::instructions::port::Port;

use crate::{
    interrupt::{cli, sti},
    nmi::{disable_nmi, enable_nmi},
};

pub const CMOS_SECOND_REGISTER: u8 = 0x0;
pub const CMOS_MINUTE_REGISTER: u8 = 0x02;
pub const CMOS_HOUR_REGISTER: u8 = 0x04;

const CMOS_COMMAND_PORT: u16 = 0x70;
const CMOS_STATUS_REGISTER_A: u8 = 0x0A;
const CMOS_STATUS_REGISTER_B: u8 = 0x0B;
const CMOS_DATA_PORT: u16 = 0x71;
const CMOS_BINARY_FORMAT_FLAG: u8 = 1 << 2;
const CMOS_UPDATE_IN_PROGRESS_FLAG: u8 = 1 << 7;

static mut COMMAND_PORT: Port<u8> = Port::new(CMOS_COMMAND_PORT);
static mut DATA_PORT: Port<u8> = Port::new(CMOS_DATA_PORT);

// https://wiki.osdev.org/CMOS#Getting_Current_Date_and_Time_from_RTC
#[derive(Debug, Clone)]
pub struct Time {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

pub fn reset_rtc() {
    while read_cmos_register(CMOS_STATUS_REGISTER_A) & CMOS_UPDATE_IN_PROGRESS_FLAG > 0 {
        core::hint::spin_loop();
    }

    write_cmos_register(CMOS_SECOND_REGISTER, 0);
    write_cmos_register(CMOS_HOUR_REGISTER, 0);
    write_cmos_register(CMOS_MINUTE_REGISTER, 0);
}

// NOTE: This function should not be used; it only works well in small ranges. For other ranges, use the sleep function provided by the tsc.
pub fn sleep_rtc(wait: u8) {
    let mut prev = Time::now().seconds;
    let mut elapsed = 0;

    while elapsed < wait {
        let current = Time::now().seconds;
        if current != prev {
            if current > prev {
                elapsed += current - prev;
            } else {
                elapsed += (60 - prev) + current;
            }
            prev = current;
        }
        core::hint::spin_loop();
    }
}

impl Time {
    pub fn now() -> Self {
        while read_cmos_register(CMOS_STATUS_REGISTER_A) & CMOS_UPDATE_IN_PROGRESS_FLAG > 0 {
            core::hint::spin_loop();
        }

        let seconds = read_datetime_reg(CMOS_SECOND_REGISTER);
        let minutes = read_datetime_reg(CMOS_MINUTE_REGISTER);
        let hours = read_datetime_reg(CMOS_HOUR_REGISTER);

        Self {
            seconds,
            minutes,
            hours,
        }
    }
    pub fn elapsed(self) -> Self {
        let end = Self::now();

        self - end
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            seconds: rhs.seconds.wrapping_sub(self.seconds),
            minutes: rhs.minutes.wrapping_sub(self.minutes),
            hours: rhs.hours.wrapping_sub(self.hours),
        }
    }
}

pub fn read_datetime_reg(reg: u8) -> u8 {
    let val = read_cmos_register(reg);

    if is_binary_format(val) {
        val
    } else {
        convert_bcd_value(val)
    }
}

pub fn read_cmos_register(register: u8) -> u8 {
    unsafe {
        cli();
        disable_nmi(register);

        #[allow(static_mut_refs)]
        let port = &mut DATA_PORT.read();
        enable_nmi();
        sti();

        port.clone()
    }
}

pub fn write_cmos_register(register: u8, value: u8) {
    unsafe {
        cli();
        disable_nmi(register);

        #[allow(static_mut_refs)]
        let _ = &mut DATA_PORT.write(value);

        enable_nmi();
        sti();
    }
}

pub fn get_cmos_format() -> u8 {
    read_cmos_register(CMOS_STATUS_REGISTER_B)
}

pub const fn is_binary_format(cmos_format: u8) -> bool {
    cmos_format & CMOS_BINARY_FORMAT_FLAG > 0
}

pub const fn convert_bcd_value(bcd: u8) -> u8 {
    ((bcd & 0xF0) >> 1) + ((bcd & 0xF0) >> 3) + (bcd & 0xf)
}
