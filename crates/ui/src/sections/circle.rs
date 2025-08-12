use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
    primitives::{Circle, PrimitiveStyle, StyledDrawable},
};

use crate::Section;

pub struct CircleSection {
    pub diameter: u32,
}

impl CircleSection {
    pub fn new(diameter: u32) -> Self {
        Self { diameter }
    }
}

impl Section for CircleSection {
    fn render(
        &self,
        screen: &mut fb::display::FbDisplay,
        point: embedded_graphics::prelude::Point,
    ) {
        let p = Point::new(point.x / 2, point.y / 2);
        let primitive_style = PrimitiveStyle::with_fill(Rgb888::BLUE);
        let circle = Circle::new(p, self.diameter);

        circle.draw_styled(&primitive_style, screen).unwrap();
    }
}
