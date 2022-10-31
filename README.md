# Lox Interpreter in Rust

Creating a interpreter for the [Lox Language](https://craftinginterpreters.com/the-lox-language.html) by following [Crafting Interpreters by Robert Nystorm](https://craftinginterpreters.com/).

Using this as an opportunity to learn rust while also learning about creating interpreters.

## How to run

Using cargo:

```sh
cargo run -- [script]
# or use REPL
cargo run
```

As a binary:

```sh
rlox [script]
# or use REPL
rlox
```

## How to run tests

```sh
cargo test
```

## TODO

- [x] Scanning
  - Improve error handling
- [ ] Representing Code (_in progress_)
- [ ] Parsing Expressions
- [ ] Evaluating Expressions
- [ ] Statements and State
- [ ] Control Flow
- [ ] Functions
- [ ] Resolving and Binding
- [ ] Classes
- [ ] Inheritance
