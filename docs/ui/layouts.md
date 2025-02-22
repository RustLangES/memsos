# Layouts
The layouts in memsos are quite simple just receive a widget and assign a position to render to that widget, although it does receive a widget but not one that implements only the Widget trait but one that implements the LayoutChild trait,

you can see some code examples in:

-   os/ui/layout/def.rs
-   os/ui/layout/vertical.rs
-   os/ui/widget/text.rs
