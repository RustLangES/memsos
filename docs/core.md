# The memsos core library

 The memsos core library is extremely simple, it is made to make memsos much more modular and also to separate the logic from the crate os/ tests, it has several abstractions like the trait Logger and Mem which are explained in the next section.

# Mem trait

The Mem trait is an abstraction of the memory read/write, it has 4 functions, check if a memory address is valid, read or write a memory address, make a memory region valid.

# Logger trait

This is even simpler, it is just a trait for core to interact with the memsos ui, in this case its log function is to interact with the bottom layout to show logs and the ui_change_test function is to change the information of a text on the screen.
