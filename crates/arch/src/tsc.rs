use crate::rtc::{reset_rtc, sleep_rtc};
use core::arch::asm;
use core::fmt::Write;
use core::time::Duration;
use fb::println;
use sync::Once;

pub static TSC_TICKS_PER_MS: Once<u64> = Once::new();

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
    let start = rdtsc();
    reset_rtc();

    sleep_rtc(5);
    let end = rdtsc();

    let ticks_per_ms = (end.wrapping_sub(start)) / 5000;

    println!("TSC ticks per ms: {}", ticks_per_ms);

    TSC_TICKS_PER_MS.call_once(|| ticks_per_ms);
}

pub fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}
