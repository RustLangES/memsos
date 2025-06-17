use std::{rc::Rc, time::Duration};

use arch::timer::Timer;
use slint::platform::software_renderer;
use uefi::proto::console::gop::BltPixel;

pub struct Platform {
    pub window: Rc<software_renderer::MinimalSoftwareWindow>,
    pub timer: Timer,
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            window: software_renderer::MinimalSoftwareWindow::new(
                software_renderer::RepaintBufferType::ReusedBuffer,
            ),
            timer: Timer::new(),
        }
    }
}

impl slint::platform::Platform for Platform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> core::time::Duration {
        self.timer.elapsed()
    }
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        use uefi::{boot::*, proto::console::gop::*};

        let gop_handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();

        let mut gop = unsafe {
            uefi::boot::open_protocol::<GraphicsOutput>(
                OpenProtocolParams {
                    handle: gop_handle,
                    agent: uefi::boot::image_handle(),
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            )
            .unwrap()
        };

        let info = gop.current_mode_info();
        let mut fb =
            vec![SlintBltPixel(BltPixel::new(0, 0, 0)); info.resolution().0 * info.resolution().1];

        self.window.set_size(slint::PhysicalSize::new(
            info.resolution().0.try_into().unwrap(),
            info.resolution().1.try_into().unwrap(),
        ));

        loop {
            slint::platform::update_timers_and_animations();

            self.window.draw_if_needed(|renderer| {
                renderer.render(&mut fb, info.resolution().0);

                let blt_fb = unsafe {
                    core::slice::from_raw_parts(fb.as_ptr() as *const BltPixel, fb.len())
                };

                gop.blt(BltOp::BufferToVideo {
                    buffer: blt_fb,
                    src: BltRegion::Full,
                    dest: (0, 0),
                    dims: info.resolution(),
                })
                .unwrap();
            });
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SlintBltPixel(BltPixel);

impl software_renderer::TargetPixel for SlintBltPixel {
    fn blend(&mut self, color: software_renderer::PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;
        self.0.red = (self.0.red as u16 * a / 255) as u8 + color.red;
        self.0.green = (self.0.green as u16 * a / 255) as u8 + color.green;
        self.0.blue = (self.0.blue as u16 * a / 255) as u8 + color.blue;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        SlintBltPixel(BltPixel::new(red, green, blue))
    }
}
