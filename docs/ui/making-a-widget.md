This is a super simple guide on how to create a widget that writes pixels per screen.

# Creating our widget

currently we only want to write one pixel per screen technically it would not be a widget because a widget is a little more complex but it is useful to see how to expand the ui in memsos.

```
pub struct Pixel {
    pub pos: (usize, usize),
    pub color: u32,
}
```

# The widget trait

This is an important yet simple trait that only requires you to have a render function and a clear function.

## implementing the Widget trait
```
impl Widget for Pixel {
    fn render(&self, writer: &mut UiWriter) {
        todo!();
    }
    fn erase(&self, writer: &mut UiWriter) {
        todo!();
    }
}
```

It is important to say that the writer is a global writer so all widgets share the same writer, which is obtained from the get_ui function.

# Writing pixels

Let's go back to our writer, where there is a write pixel function written like this

```rust
fn write_pixel(&mut self, x: u64, y: u64, color: u32) 
```

So, adapting our render function, it would look like this

```rust
impl Widget for Pixel {
    fn render(&self, writer: &mut UiWriter) {
        writer.write_pixel(self.pos.0, self.pos.1, self.color);
    }
    fn erase(&self, writer: &mut UiWriter) {
        todo!();
    }
}

```

Exercise: implement the clear function on your own
