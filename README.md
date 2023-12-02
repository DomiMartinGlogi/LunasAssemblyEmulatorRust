# Luna's Assembly Emulator

[![Rust](https://github.com/DomiMartinGlogi/LunasAssemblyEmulatorRust/actions/workflows/rust.yml/badge.svg)](https://github.com/DomiMartinGlogi/LunasAssemblyEmulatorRust/actions/workflows/rust.yml)

## General

Goal of this Project is to implement a simple custom Assembly Language, Compiler and Architecture
in the Rust Programming Language.

Luna's Assembly Emulator Rust is shortened to lear for the naming of it's components, like for example
the instruction set crate is `lear_instruction_set`.

## Usage

The entrypoint of a program written in this language is to begin with the label:
`start:{`, this denotes the main (starting) function, it ends with `}` on a new line.
All functions work like this, `[name]:{ ... }` where `[name]` would be the functions name. 

These blocks CANNOT be nested!

Comments can be begun with this syntax: `[code] ; [comment]` the comment ends at the next New Line.