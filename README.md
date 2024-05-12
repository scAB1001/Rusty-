# Rust

## Intro

Rust is a programming language used for systems programming where performance and correctness are high priorities.

_Why?_
- [A Stack Overflow Favourite](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)

### Why Care About Rust?

- **Memory Safety Without Garbage Collection:** Rust provides memory safety guarantees via its unique ownership model and borrowing system, without needing a garbage collector. This leads to performance akin to that of C or C++, but with added safety.
- **Concurrency Without Data Races:** Rust's type system and ownership rules prevent data races, making concurrent programming easier and safer. It enforces thread safety guarantees, thus errors like race conditions can be caught at compile time.
- **Performance:** Rust is designed to be as fast as traditionally fast languages like C++. It gives precise control over the usage of resources and memory without overhead.
- **Tooling:** Rust includes powerful tools such as `cargo` for project management, `rustfmt` for code formatting, and `clippy` for linting, which enhance the coding experience.
- **Cross-platform Development:** Rust supports cross-compilation, allowing development on one platform (like Windows) and deployment on another (like Linux) seamlessly.
- **Growing Ecosystem:** Although younger than languages like Python or Java, Rust's ecosystem is rapidly expanding, with a wide array of libraries and frameworks being developed.

### Rust's Strengths Compared to Other Languages

- **Compared to C/C++:** Rust offers better memory safety without sacrificing performance, thanks to its ownership model. This reduces the likelihood of bugs and security vulnerabilities associated with improper memory handling.
- **Compared to Python/JavaScript:** Rust provides much higher performance and efficiency, suitable for systems-level and embedded programming, where resource constraints are critical.
- **Compared to Go:** Rust gives finer control over system resources and performance, and it has stricter compile-time checks for ensuring thread safety.

### When is Rust Used?

- **Systems Programming:** Ideal for writing operating systems, game engines, and other systems where low-level memory management is critical.
- **Web Development:** With frameworks like Actix and Rocket, Rust is becoming popular for backend web development, offering performance benefits over traditional scripting languages.
- **Embedded Programming:** Its low overhead and memory safety features make Rust suitable for embedded systems development.
- **Cryptocurrency and Blockchain:** Rust's robustness and performance characteristics make it a preferred choice for blockchain implementation.
- **Networking and Infrastructure:** Rust's safety and performance features are advantageous in developing networking devices and protocols, data storage systems, and high-performance computing applications.

Rust is particularly compelling for developers looking to optimize performance while ensuring high degrees of safety and concurrency, making it increasingly popular in professional and critical system applications.

### Installation
Install Rust with the rustup toolset.
```bash
# Install Rust on Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal and check the version to see these changes take effect 
rustc --version
rustc update  # There are new versions every 6 weeks !

# To view the Rust documentation
rustup doc

```

### Program
Following: [Rust - by example](file:///home/baz/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/rust-by-example/index.html)
```bash
# Create a new .rs file
# Compile
rustc file.rs

# Run
./file

```

#### Cargo
When you install Rust with rustup, the toolset includes the rustc compiler, the rustfmt source code formatter, and the clippy Rust linter. 
You'll find that you end up using cargo for just about everything.
```bash
# Check everything is installed
cargo --version

```
You also get Cargo, the Rust package manager, to help download Rust dependencies and build and run Rust programs. 

A good way to create your first Rust program is to use Cargo to scaffold a new project.
This will create a simple Hello World program along with a default Cargo.toml dependency file. 
```bash
# Pass cargo + dir/ where you'd like to create the project
cargo new hello_world

# Build your Rust project
cargo build

# Run your Rust project 
cargo run

# -or you can run it manually in the terminal 
.\target\debug\program_name

```
