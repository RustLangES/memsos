use core::{hint::spin_loop, time::Duration};

use acpi::hpet::{HPET_MAIN_COUNTER_REGISTER, HPET_STATE, read_hpet};

pub fn read_main_counter() -> u64 {
    read_hpet(HPET_MAIN_COUNTER_REGISTER)
}

pub fn frequency() -> u64 {
    HPET_STATE.frequency
}

pub fn sleep_hpet(duration: Duration) {
    let nanos = duration.as_nanos() as u64;
    let ticks = read_main_counter() + ((nanos * 1_000_000) / HPET_STATE.period);

    while read_main_counter() < ticks {
        spin_loop();
    }
}
