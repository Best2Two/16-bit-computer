# Hack Assembler

This directory contains my implementation of the Hack Assembler, developed in Rust as part of the **Nand2Tetris** (Part One) course.

## Overview

This is an **educational assembler** designed specifically for the Hack computer architecture. Its purpose is to translate Hack assembly language (`.asm` files) into binary machine code (`.bin` files) that can be executed by the Hack CPU.

This project was built from scratch following the specifications provided in the Nand2Tetris course. It serves to demonstrate how human-readable assembly instructions are parsed, how symbols are resolved into memory addresses, and how the final binary instructions are constructed.

### Features
The assembler performs a two-pass translation process and handles:
- Stripping out whitespaces and inline comments.
- Parsing A-instructions (address instructions like `@100` or `@LABEL`).
- Parsing C-instructions (computation instructions like `D=D+A;JMP`).
- Resolving predefined symbols (e.g., `R0`-`R15`, `SP`, `LCL`, `SCREEN`, `KBD`).
- Managing a symbol table for user-defined labels (e.g., `(LOOP)`) and variables.

## How to Build and Run

This project is written in Rust and uses Cargo for dependency management and building.

### Prerequisites
- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.

### Running the Assembler

You can run the assembler directly on an assembly file using `cargo run`.

Pass the path of the `.asm` file as an argument:

```bash
cd Assembler
cargo run -- <path_to_your_file.asm>
```

**Example:**
If you want to assemble a file located at `examples/Add.asm`:

```bash
cargo run -- examples/Add.asm
```

The assembler will process the file and generate a new `.bin` file (e.g., `Add.asm.bin`) containing the translated 16-bit binary instructions in the current working directory.

### Building

To compile the assembler into a standalone executable without running it:

```bash
cargo build --release
```

The compiled binary will be available in the `target/release/` directory.
