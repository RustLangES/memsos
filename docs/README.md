# Welcome to the memsos docs

Here you will see how to compile memsos from scratch, and you will also see some details of how memsos works and what it does behind the scenes.

# Getting started (without Nix)

To get started with memsos without nix you will need some programs to compile it that must be on your computer, these are:

- the rust toolchain (make sure you are on nightly)
- git
- make
- just
- xorriso
- curl
if you have all of these programs on your computer you can run ``just`` and memsos will compile normally.

(watch out if you don't have qemu cargo run it won't work)

# Getting started (with Nix)

With nix it's a little bit easier to start developing by entering a devShell with

```
nix develop
```

You can simply build the project with nix build and if you want to see the list of targets you can use nix build .#list
