# Rox

A high-performance Rust implementation of the Lox programming language from Robert Nystrom's excellent book [Crafting Interpreters](https://craftinginterpreters.com/).

Rox is a bytecode virtual machine interpreter that compiles Lox source code into bytecode and executes it on a stack-based VM, similar to the `clox` implementation from Part III of the book.

## Features

Rox implements the complete Lox language specification, including:

- **First-class functions** with closures and upvalue capturing
- **Object-oriented programming** with classes, methods, and constructors
- **Class inheritance** with the `super` keyword for superclass method access
- **Lexical scoping** with global and local variables
- **Control flow** including `if/else`, `while`, and `for` loops
- **Built-in types**: numbers (f64), strings, booleans, and nil
- **Native functions** like `clock()` for system integration
- **Performance optimizations** including specialized invoke instructions for method calls

### Performance Optimizations

Rox includes several performance enhancements beyond the basic implementation:

- **Invoke Optimization**: Method calls use a specialized `Invoke` instruction that combines property lookup and method invocation in a single operation, eliminating the creation of short-lived `BoundMethod` objects
- **Optimized Development Builds**: Development builds run with optimization level 3 for faster testing and iteration

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.56 or later (2021 edition)

### Building from Source

```bash
git clone <repository-url>
cd rox
cargo build --release
```

The compiled binary will be available at `target/release/rox`.

## Usage

### REPL Mode

Run Rox without arguments to start an interactive REPL (Read-Eval-Print Loop):

```bash
cargo run
```

```
Welcome to Lox REPL!
> print "Hello, world!";
Hello, world!
> var x = 42;
> print x * 2;
84
```

### Running Lox Scripts

Execute a Lox source file:

```bash
cargo run examples/function/recursion.lox
```

Or with the compiled binary:

```bash
./target/release/rox examples/function/recursion.lox
```

## Language Examples

### Functions and Recursion

```lox
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}

print fib(8); // 21
```

### Classes and Inheritance

```lox
class Animal {
  speak() {
    print "Some sound";
  }
}

class Dog < Animal {
  speak() {
    print "Woof!";
  }

  callSuper() {
    super.speak();
  }
}

var dog = Dog();
dog.speak();      // Woof!
dog.callSuper();  // Some sound
```

### Closures

```lox
fun makeCounter() {
  var count = 0;
  fun counter() {
    count = count + 1;
    return count;
  }
  return counter;
}

var counter = makeCounter();
print counter(); // 1
print counter(); // 2
print counter(); // 3
```

### Class Constructors

```lox
class Person {
  init(name, age) {
    this.name = name;
    this.age = age;
  }

  greet() {
    print "Hi, I'm " + this.name + "!";
  }
}

var person = Person("Alice", 30);
person.greet(); // Hi, I'm Alice!
```

## Testing

The project includes a comprehensive test suite with hundreds of test cases covering all language features.

### Running All Tests

```bash
cargo test -- --test-threads=1
```

Note: Tests must run single-threaded (`--test-threads=1`) because they capture and verify stdout output.

### Running Specific Tests

```bash
cargo test test_function_recursion -- --test-threads=1
cargo test test_class -- --test-threads=1
cargo test test_inheritance -- --test-threads=1
```

### Test Organization

- `examples/` - Contains Lox source files organized by feature (assignment, functions, classes, etc.)
- `tests/` - Rust integration tests that execute the examples and verify output

## Architecture

Rox follows a classic bytecode VM architecture:

```
Source Code → Scanner → Parser → Bytecode → VM → Execution
```

### Core Components

- **Scanner** (`scanner.rs`) - Lexical analysis, tokenizes source code
- **Parser** (`parser.rs`) - Compiles tokens into bytecode chunks
- **Chunk** (`chunk.rs`) - Contains bytecode instructions and constant pool
- **VM** (`vm.rs`) - Stack-based bytecode interpreter with call frame management
- **Value** (`value.rs`) - Tagged union representing runtime values
- **Function** (`function.rs`) - Function objects and native function interface
- **Class** (`class.rs`) - Class objects and method tables
- **Closure** (`closure.rs`) - Closure objects with upvalue management
- **Upvalue** (`upvalue.rs`) - Captured variables for closures
- **CallFrame** (`call_frame.rs`) - Function call stack management
- **CompilationContext** (`compilation_context.rs`) - Variable scoping during compilation

### Instruction Set

The VM uses a stack-based instruction set including:

- Arithmetic: `Add`, `Subtract`, `Multiply`, `Divide`, `Negate`
- Comparison: `Equal`, `Greater`, `Less`
- Logical: `Not`
- Variables: `DefineGlobal`, `GetGlobal`, `SetGlobal`, `GetLocal`, `SetLocal`
- Control flow: `Jump`, `JumpIfFalse`, `Loop`
- Functions: `Call`, `Return`, `Closure`
- Classes: `Class`, `Method`, `Invoke`, `GetProperty`, `SetProperty`
- Inheritance: `Inherit`, `GetSuper`, `SuperInvoke`

## Development

### Code Style

```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run linter
cargo clippy
```

### Project Structure

```
rox/
├── src/           # Source code
├── examples/      # Lox test files organized by feature
├── tests/         # Rust integration tests
└── Cargo.toml     # Project configuration
```

## Resources

- [Crafting Interpreters](https://craftinginterpreters.com/) - The definitive guide to the Lox language
- [Lox Language Specification](https://craftinginterpreters.com/appendix-i.html)
