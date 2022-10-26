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

## TODO

**Interpreter**

- [ ] Scanning (_in progress_)
- [ ] Representing Code
- [ ] Parsing Expressions
- [ ] Evaluating Expressions
- [ ] Statements and State
- [ ] Control Flow
- [ ] Functions
- [ ] Resolving and Binding
- [ ] Classes
- [ ] Inheritance

**Bytecode Virtual Machine**

- [ ] Chunks of Bytecode
- [ ] A Virtual Machine
- [ ] Scanning on Demand
- [ ] Compiling Expressions
- [ ] Types of Values
- [ ] Strings
- [ ] Hash Tables
- [ ] Global Variables
- [ ] Local Variables
- [ ] Jumping Back and Forth
- [ ] Calls and Functions
- [ ] Closures
- [ ] Garbage Collection
- [ ] Classes and Instances
- [ ] Methods and Initializers
- [ ] Superclasses
- [ ] Optimization
