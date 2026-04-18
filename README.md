# Forge

Forge is a compiled, systems-level programming language built in Rust. It takes inspiration from two worlds:

- **Rust** expressive types, pattern matching, and a modern toolchain.
- **C++** low level control, familiar systems programming patterns, and zero-cost abstractions

The goal is to explore what a language looks and feels like when you cherry-pick the best ideas from both.

> [!NOTE]
> This is my first time using rust

## Goals

- [ ] Lexer / Tokenizer
- [ ] Parser & AST
- [ ] Type system
- [ ] Code generation (LLVM or custom IR)
- [ ] Basic standard library

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Cargo (comes with Rust)

### Build & Run

```
git clone https://github.com/Ggjorven/Forge.git
cd Forge
cargo build --release

cargo run -- path/to/file.fg
```

## Syntax

```rs
using system.io;

int32 main() 
{
    let msg /*: string8 */ = "Hello from Forge!";
    io.print(msg);
}
```

> Syntax is not final and will evolve as the language takes shape.

## Inspiration

- The [Rust programming language](https://www.rust-lang.org/)
- The [C++ standard](https://isocpp.org/)

## License

This project is licensed under the GNU GPLv2.0 LICENSE. See [LICENSE](LICENSE.txt) for details.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.
