use crate::rtc::{reset_rtc, sleep_rtc};
use core::arch::asm;
use core::fmt::Write;
use core::time::Duration;
use fb::println;
use sync::Once;

pub static TSC_TICKS_PER_MS: Once<u64> = Once::new();

pub struct Timestamp {
    pub minutes: u64,
    pub seconds: u64,
    pub hours: u64,
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}

pub struct Instant {
    start_tick: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            start_tick: rdtsc(),
        }
    }
    pub fn elapsed(&self) -> Duration {
        let end = rdtsc();
        let elapsed_ms = (end - self.start_tick) / *TSC_TICKS_PER_MS;

        Duration::from_millis(elapsed_ms)
    }
    pub fn to_timestamp(&self) -> Timestamp {
        let mut seconds = self.elapsed().as_secs();
        let mut minutes = 0;
        let mut hours = 0;

        if seconds >= 60 {
            minutes = seconds / 60;
            seconds = seconds % 60;
        }

        if minutes >= 60 {
            hours = minutes / 60;
            minutes = minutes % 60;
        }

        Timestamp {
            minutes,
            seconds,
            hours,
        }
    }
}

pub fn sleep(time: Duration) {
    let ms = time.as_millis() as u64;
    let relative_ticks_to_wait = ms.wrapping_mul(*TSC_TICKS_PER_MS);

    let mut tsc_ticks = rdtsc();
    let ticks_to_wait = relative_ticks_to_wait.wrapping_add(tsc_ticks);

    while tsc_ticks < ticks_to_wait {
        tsc_ticks = rdtsc();
        core::hint::spin_loop();
    }
}

pub fn calibrate_tsc() {
    reset_rtc();

    let start = rdtsc();
    sleep_rtc(5);
    let end = rdtsc();

    let ticks_per_ms = (end.wrapping_sub(start)) / 5000;

    println!("TSC ticks per ms: {}", ticks_per_ms);

    TSC_TICKS_PER_MS.call_once(|| ticks_per_ms);
}

pub fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}
