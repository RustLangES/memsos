# Rendering a widget

This part is quite simple when rendering a widget we just run the render function of the widget

```
let pixel = Pixel { pos: (200, 200), color: 0xFF };
render!(&pixel);
```

we can also render multiple widgets
```
render!(&widget1, &widget2, ...)
```

# Deleting a widget

Well we have our widget drawn, but now I want it to be removed from the screen well it's as simple as using the erase macro

```
erase!(&pixel);
```

# Cleaning the entire screen

If you want to go to the extreme and want to erase everything on the screen you can use the clean function which is used several times in kernel/main.rs.
