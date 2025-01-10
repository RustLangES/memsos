use crate::ui::widget::Widget;

pub struct Line {
    from: (isize, isize),
    to: (isize, isize),
}

#[inline]
pub fn line(from: (isize, isize), to: (isize, isize)) -> Line {
   Line {
     from,
     to,
   } 
}

impl Widget for Line {
    fn render(&self, writer: &mut super::writer::UiWriter) {
        let dx = self.to.0 - self.from.0;
        let dy = self.to.1 - self.from.1;

        let steps = match isize::abs(dx) > isize::abs(dy) {
            true => isize::abs(dx),
            false => isize::abs(dy),
        };

        let x_inc = dx / steps;
        let y_inc = dy / steps;

        let mut p = (self.from.0, self.from.1);

        for _i in 0..=steps {
            writer.write_pixel(p.0.try_into().unwrap(), p.1.try_into().unwrap(), 255);
            p.0 += x_inc;
            p.1 += y_inc;
        }

    }
    fn erase(&self, writer: &mut super::writer::UiWriter) {
        unimplemented!();
    }
}

impl Line {
    pub fn draw() {}
}
