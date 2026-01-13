# 🦀 🎮 Fly with Rust 

- **FlyRust** is a **game** developed in **Rust** using **egui**, following the **MVVM architecture**.

- **`Fly with Rust`** is a **Rust** project named ***`FlyRust`***, created as a hands-on practice for learning Rust.
The idea is to learn Rust by building and flying with it, hence the name Fly Rust.

- This project is designed with a strong focus on architecture, correctness, and scalability, rather than being just a simple demo.

- Demo Video and Screenshot  
To be added...

## Goals

- Practice idiomatic Rust
- Learn project-level architecture, not just syntax
- Apply **MVVM (Model–View–ViewModel)** concepts in **Rust** with **egui**
- Build a codebase that is:
  - testable
  - extensible
  - maintainable
- Cross-Platform Support
  - Development is primarily done on **macOS**.
  - Tested and verified on **Windows 10/11**.
  - Core logic is designed to be platform-agnostic
  - No OS-specific behavior in the engine layer
  - Platform differences are isolated when necessary
  
---

## Architecture
The overall architecture is documented in ***`docs/architecture.md`***.  

- Workspace Structure brief
This project is organized as a Cargo workspace with the following crates:
  - flyrust  
Binary crate and main entry point of the application.
Responsible for application startup, wiring components, and runtime configuration.
  - engine  
Core game engine crate implementing the MVVM architecture.  
Contains:
    - domain logic
    - state management
    - view models
    - engine-level abstractions
  - util  
    - Shared utility library used across the workspace.  
    - Contains reusable helpers, common types, and general-purpose functionality.
---

## ⚙️ Build 

At workspace root:

```bash
# build workspace
cargo build
```

---

## ▶️ Run CLI (binary crate)

```bash
cargo run -p flyrust
```

* `flyrust` is the binary crate.
* `main.rs` is the main entry.
