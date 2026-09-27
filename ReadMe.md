# Asteroid Rush

A classic space asteroid shooter game where you pilot a spaceship and shoot down asteroids.

## Tech Stack

- [Rust](https://doc.rust-lang.org/book/) : systems programming language
- [Macroquad](https://macroquad.rs/) : simple and easy to use game library

## Why Rust + Macroquad?

Rust was chosen as a way to learn Rust. [Macroquad](https://docs.rs/macroquad)
([repo](https://github.com/not-fl3/macroquad)) was chosen because, at the
time this repo was being developed, it was the best option for a simple game
like this.

<details>
<summary>Alternatives considered</summary>

### Rust

Excellent language for anyone coming from C/C++: memory safety without a GC,
great tooling (`cargo`, `clippy`, `rustfmt`), but compile times can be
slower, especially for large graphics/ECS crates.

- **[macroquad](https://docs.rs/macroquad)** ([repo](https://github.com/not-fl3/macroquad)) : chosen for this project. Simple, dependency-light, fast to compile, and enough for a small/simple game or game jam.
- **[Bevy](https://bevyengine.org)** ([repo](https://github.com/bevyengine/bevy)) : A full-featured, data-driven engine built around ECS. More powerful and batteries-included than macroquad (2D/3D, asset pipeline, plugins), but with a steeper learning curve and heavier compile times — overkill for a project this size.
- **raylib/sfml Rust bindings** ([raylib-rs](https://github.com/deltaphc/raylib-rs), [rust-sfml](https://github.com/jeremyletang/rust-sfml)), These wrap C/C++ libraries rather than being Rust-native, so the ergonomics and safety guarantees can suffer at the FFI boundary. Prefer macroquad or Bevy unless you specifically need one of these libraries.

### Go

Easy to learn, fast to build, and comes with excellent tooling out of the
box. [Official docs](https://go.dev/doc/)

- **[Ebiten](https://ebitengine.org)** ([repo](https://github.com/hajimehoshi/ebiten)) : A dead-simple 2D game library for Go. Small API surface, cross-platform (desktop, mobile, browser via WASM), good for quick 2D prototypes. A reasonable alternative, but Rust's tooling and safety guarantees were preferred for this project.

### C/C++

Mature, battle-tested libraries, but weaker tooling than Go/Rust: no
built-in package manager (needs vcpkg/Conan/etc.), and splitting code
across multiple files is more friction than it should be without discipline
around headers.

- **[Raylib](https://www.raylib.com)** ([repo](https://github.com/raysan5/raylib)) : A simple, easy-to-use library for game programming, with a very approachable C API and bindings for many other languages. Macroquad's API is directly inspired by it.
- **[SFML](https://www.sfml-dev.org)** ([repo](https://github.com/SFML/SFML)) : Simple and Fast Multimedia Library; object-oriented C++ API covering graphics, audio, windowing, and networking.
- **OpenGL** : A low-level graphics API. Too low-level for most game dev: requires a large amount of boilerplate, is hard to debug, and slows down development compared to the options above.

</details>

## Development

### Option 1: Install Rust locally (recommended)

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install the recommended VS Code extension: `rust-lang.rust-analyzer`
3. Run the game:

   ```sh
   cargo run
   ```

### Option 2: Dev Container

Running a Dev Container works fine, but
since this is a UI app, one will need extra UI-related setup on the host OS
(e.g. an X server on Windows) to see the window.