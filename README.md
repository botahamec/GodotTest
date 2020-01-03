# GodotTest
A set of tests for Godot I've been using

## Compiling
Compiling the GDscript stuff is pretty self-explanatory, but compiling the Rust code requires some manual labor. There are two lib sections in the Cargo.toml file. One of which is not commented out. You must uncomment the one you need to compile and comment out the rest. Unfortunately Cargo will not compile more than one library at a time.

## Running
There are two scenes in the main directory.

* Test - just a gravity simulator
* Fibonacci - compares the speed of Rust versus Godot at calculating Fibonacci numbers

Simply open the scene you'd like to try and play. No main scene has been set.
