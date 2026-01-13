# Architecture Overview

This document describes the high-level architecture, design principles,
and crate responsibilities of this Rust workspace.

---

## Goals & Non-Goals

### Goals

* Clear separation between **business logic** and **interfaces**
* Reusable, testable core libraries
* Minimal coupling between crates
* Predictable dependency flow

### Non-Goals

* Not a monorepo with tightly coupled crates
* Not optimizing for premature performance
* Not exposing internal crates as public APIs unless explicitly stated

---

## Workspace Structure


```
my_workspace/               # workspace root
├── Cargo.toml              # Cargo.toml
├── Cargo.lock              # Cargo.lock
├── README.md               # workspace overview
├── CONTRIBUTING.md         # Contributing guide
├── LICENSE                 # License
├── LICENSE-MIT             # License MIT
├── LICENSE-APACHE-2.0      # License Apache-2.0
├── docs/
│   ├── docing-style.md     # Documentation style guide
│   └── architecture.md     # Architecture overview
├── crates/
│   ├── util/               # library crate
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── src/lib.rs      # fixed lib.rs to declare lib
│   │   └── tests/          # integration test
│   │       └── integration_test.rs
│   ├── engin/               # library crate
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── src/lib.rs      # fixed lib.rs to declare lib
│   │   └── tests/          # integration test
│   │       └── integration_test.rs
│   └── flyrust/            # binary crate
│       ├── Cargo.toml
│       ├── README.md
│       └── src/main.rs
├── target/                 # cargo build output folder
├─- assets/                 # assets: include to embed binary
│       ├── image/          # Image
│       └── screenshots/    # Screenshots
├── Contents/               # macOS APP structure
│   └── Resources/          # Resources
│       └──assets/          # assets: image, audio, fonts, and etc..
│           └── i18n        # Internationalization
└── dist/                   # Distribution folder               
```

---

## Crate Responsibilities

### `lib`

**Purpose**

* Contains core domain logic and business rules

**Characteristics**

* No direct I/O (no filesystem, network, stdin/stdout)
* Deterministic and testable
* Exposes a stable public API

**Allowed dependencies**

* `std`
* Pure logic crates (e.g. `thiserror`, `serde`)

**Not allowed**

* CLI parsing
* Logging initialization
* Environment access

---

### `app`

**Purpose**

* User-facing command-line interface

**Responsibilities**

* Argument parsing
* Input/output
* Error presentation
* Calling library APIs

**Design rule**

> The `app` crate should be thin.
> Most logic must live in libraries.

---

## Dependency Rules

The dependency direction must always flow **inward**:

```text
app ───▶ lib
```

🚫 Reverse dependencies are not allowed.

---

## Error Handling Strategy

* Libraries return `Result<T, E>`
* Errors are domain-specificnaries decide how errors are displayed

---

## Testing Strategy

| Test Type         | Location     | Purpose                       |
| ----------------- | ------------ | ----------------------------- |
| Unit tests        | `src/lib.rs` | Validate internal logic       |
| Integration tests | `tests/`     | Validate public API           |
| Doc tests         | Rustdoc      | Validate usage examples       |
| app tests         | `app/tests/` | Validate user-facing behavior |

---

## Documentation Strategy

* Public APIs **must** have rustdoc comments
* Examples are preferred over prose
* Architecture decisions live in `/docs`

---

## Future Considerations

* Potential crate extraction
* Public API stabilization
* CI enforcement of linting and documentation coverage
