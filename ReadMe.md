# Asteroid Rush

A classic space asteroid shooter game where you pilot a spaceship and shoot down asteroids.

## Demo

- [Watch on YouTube](https://youtu.be/hFXr7JuFGW0)
- Local video file: [`demo/asteroid_rush_game_demo.mp4`](./demo/asteroid_rush_game_demo.mp4)

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

Running a Dev Container works fine, but since this is a UI app with sound,
you'll need some extra setup on the host OS to actually see the window and
hear audio since the container has no display or sound device of its own.

**On Windows:**

- **Display** : install [VcXsrv](https://sourceforge.net/projects/vcxsrv/)
  - Launch it with XLaunch
  - On the last page, enable **"Disable access control"**
  - That's it, the container connects to it automatically
- **Audio** : works out of the box on Windows 11 with WSL2, nothing to
  install or configure
  - The container has no sound card, so audio is routed through WSLg
    (Windows' built-in Linux GUI support), which runs a real audio
    server inside one of your WSL distros
  - The dev container auto-detects which distro that is (see
    `detect-wsl-distro.ps1`), no manual setup needed

> **Note:** audio and input aren't talking to real hardware directly here,
> they're forwarded over TCP/WSLg. Expect a bit more lag and occasional
> audio glitches versus running natively. Fine for development, not how
> the game actually feels to play.

**On macOS/Linux:**
- The container always builds and runs fine
- Display/audio default to the Windows setup above, so they won't work
  out of the box
- Both are overridable in `.devcontainer/.env`:
  - `CONTAINER_DISPLAY` : set to `:0` to use your own X server, via the
    X11 socket already mounted in `docker-compose.yaml`
  - `WSLG_PULSE_SOURCE` : point at your own PulseAudio socket (e.g.
    `/run/user/1000/pulse/native` on most Linux desktops)
- Not tested on either platform

## Releases

- Prebuilt binaries live in [`releases/`](releases/), one subfolder per
  platform
- Sprites and audio are embedded into the binary at compile time (via
  `include_bytes!`), so each one is fully standalone, no extra files
  needed alongside it
- **Windows:** [`releases/windows/`](releases/windows/) : download the
  latest `.exe` and run it directly

### Building a new Windows release

```powershell
cargo build --release
New-Item -ItemType Directory -Force -Path releases\windows | Out-Null
Move-Item target\release\asteroid_rush.exe "releases\windows\asteroid_rush_windows_$([DateTime]::UtcNow.ToString('yyyyMMdd')).exe"
```

- Builds the optimized binary
- Drops it into `releases/windows/`
- Names it with today's date (UTC), e.g. `asteroid_rush_windows_20261002.exe`