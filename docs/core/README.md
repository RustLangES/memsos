# The memsos core library

The memsos core library is one of the most important libraries of memsos, in this library is the code of the memory tests.

# Logger and Mem trait

## Logger
The trait logger is the easiest to use, because it is simply to display information on the screen, in memsos-os Logger is an abstraction of the memsos ui, specifically the Debug layout and the text widget.


## Mem trait

This is a bit more complex, this is for accessing memory and writing to memory, now not the trait Mem is not for memory checking but for basic operations, this is made up of four functions, write and read for writing and reading respectively, check is quite simple it checks if a memory address is valid and correctly aligned and then there is parse which makes a memory region valid.

# What is the reason for the existence of core?
It is quite simple:
- Modularize memsos
- You wanted to expand memsos to other presentations so to speak and that presentation works differently and there are some extra steps to read/write to memory.
