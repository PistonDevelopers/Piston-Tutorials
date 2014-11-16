# Piston-Tutorials [![Build Status](https://travis-ci.org/PistonDevelopers/Piston-Tutorials.svg)](https://travis-ci.org/PistonDevelopers/Piston-Tutorials)

This is a repository for examples of Piston projects that have are
accompanied by written tutorials explaining core concepts for that
tutorial.

## Current Tutorials

#### [Getting Started With Piston](./getting-started)
A "tutorial" with instructions on compiling and running a very
simple Piston.

## Planned Tutorials

* rust-graphics
* piston

## Making changes to the tutorials
Because most of the tutorials will contain heavy amounts of 
code, TyOverby developed a markdown pre-processor that takes 
`readme.dev.md` files and includes code from the surrounding 
project.  This way you don't need to make a change in the code
for the tutorial and then also make the same change in the 
readme.md file; the preprocessor will do that for you!

In order to run the pre-processor, simply invoke `cargo run` 
from the root directory (not the sub-tutorial directory) and 
it will rebuild all the markdown files that it knows about.

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
