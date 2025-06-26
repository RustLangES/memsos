use core::time::Duration;

pub struct Timer {
    pub start_tick: u64,
    pub timer_freq: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_tick: timer_tick(),
            timer_freq: timer_freq(),
        }
    }
    pub fn elapsed(&self) -> Duration {
        Duration::from_secs_f64(
            (self.get_tick() as f64 - self.start_tick as f64)
                / self.timer_freq as f64,
        )
    }
    pub fn reset(&mut self) {
        self.start_tick = timer_tick();
    }
    pub fn get_tick(&self) -> u64 {
        timer_tick()
    }
}

fn timer_tick() -> u64 {
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::x86::_rdtsc()
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut ticks: u64;
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) ticks);
        ticks
    }
}

fn timer_freq() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let start = timer_tick();
        uefi::boot::stall(1000);
        let end = timer_tick();
        (end - start) * 1000
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut freq: u64;
        core::arch::asm!("mrs {}, cntfrq_el0", out(reg) freq);
        freq
    }
}
