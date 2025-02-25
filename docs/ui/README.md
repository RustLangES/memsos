# Memsos Ui

The memsos ui is one of the most important part of memsos, it is what the user sees, there are two parts the Writer, Widgets and the Layout the writer is really simple just write pixels on screen the Widgets are parts of the ui these have to implement a widget trait and receive a mutable reference from the writer, example:

```rust
pub struct MyWidget {}

impl Widget for MyWidget {
    fn render(&self, writer: &mut UiWriter) {
       // ...    
    }
    fn erase(&self, writer: &mut UiWriter) {
      // ...
    }

}
```

## Layouts

The memsos layouts are quite easy to use and create what it does is that it receives a widget and assigns it a position dynamically for more information see vertical.rs
