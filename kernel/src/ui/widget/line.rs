use crate::ui::widget::Widget;
use crate::ui::writer::UiWriter;

pub struct Line {
    from: (isize, isize),
    to: (isize, isize),
}

#[inline]
pub fn line(from: (isize, isize), to: (isize, isize)) -> Line {
    Line { from, to }
}

impl Widget for Line {
    fn render(&self, writer: &mut UiWriter) {
        let mut p = (self.from.0, self.from.1);

        let (x_inc, y_inc, steps) = self.calculate_distance();

        core::iter::repeat(()).take(steps + 1).for_each(|_| {
            writer.write_pixel(p.0.try_into().unwrap(), p.1.try_into().unwrap(), 255);
            p.0 += x_inc;
            p.1 += y_inc;
        });
    }
    fn erase(&self, writer: &mut UiWriter) {
        let mut p = (self.from.0, self.from.1);

        let (x_inc, y_inc, steps) = self.calculate_distance();

        core::iter::repeat(()).take(steps + 1).for_each(|_| {
            writer.write_pixel(p.0.try_into().unwrap(), p.1.try_into().unwrap(), 0);
            p.0 += x_inc;
            p.1 += y_inc;
        });
    }
}

impl Line {
    // 0: x_inc
    // 1: y_inc
    // 2: steps
    fn calculate_distance(&self) -> (isize, isize, usize) {
        let dx = self.to.0 - self.from.0;
        let dy = self.to.1 - self.from.1;

        let steps = isize::abs(dx).max(isize::abs(dy));

        let x_inc = dx / steps;
        let y_inc = dy / steps;

        (x_inc, y_inc, steps.try_into().unwrap())
    }
}
