# Ziggy

Ziggy is a transpiler project written in Rust that converts Rust source code into equivalent Zig code. The goal of Ziggy is to facilitate translation between these two languages, enabling interoperability and easier migration for developers working across Rust and Zig ecosystems.


## Project Structure

- **Tokenizer:** The core component implemented so far, responsible for lexing Rust source into tokens.
- **Token Types:** Supports a range of token types including identifiers, numbers, operators, and various bracket types.
- **Input Handling:** Processes Rust source code line by line, ensuring whitespace and blank lines are handled correctly.

## Usage

The transpiler currently includes a command-line tool that tokenizes Rust source files. To use it, follow these steps:

1. **Run with Cargo**

    You can run the tokenizer directly using Cargo by passing the path to the Rust source file or project folder as an argument:

    ```bash
    rustc main.rs -o ziggy
    ./ziggy  /path/to/your/rust/source.rs
    ```

2. **What it does**

    The program will:

    - Read the Rust source file at the specified path (via `finder::read_rust_file`)
    - Tokenize the contents using the tokenizer module
    - Print the list of tokens to standard output in a readable debug format

3. **Sample output**

    Output will look like:

    ```text
    [
        Identifier(
            "fn",
        ),
        Identifier(
            "main",
        ),
        ParenOpen,
        ParenClose,
        BraceOpen,
        // ...more tokens
    ]
    ```
