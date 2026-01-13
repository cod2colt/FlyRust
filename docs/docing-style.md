# Rust Documentation Style Guide

This document defines the documentation standards for this Rust workspace.
All contributors must follow these rules to ensure consistency, readability,
and high-quality API documentation.

## Table of Contents
<!-- TOC -->
- [Rust Documentation Style Guide](#rust-documentation-style-guide)
  - [Table of Contents](#table-of-contents)
  - [1. Documentation Comment Style](#1-documentation-comment-style)
    - [✅ Use Line-Based Doc Comments (Required)](#-use-line-based-doc-comments-required)
    - [❌ Do NOT Use Block-Based Doc Comments](#-do-not-use-block-based-doc-comments)
  - [2. What Must Be Documented](#2-what-must-be-documented)
  - [3. Documentation Examples by Item Type](#3-documentation-examples-by-item-type)
    - [3.1 Struct Documentation](#31-struct-documentation)
    - [3.2 Enum Documentation](#32-enum-documentation)
    - [3.3 Trait Documentation](#33-trait-documentation)
    - [3.4 impl Block Documentation](#34-impl-block-documentation)
  - [4. Required Sections for Public APIs](#4-required-sections-for-public-apis)
    - [Example: Fully Documented Public Function](#example-fully-documented-public-function)
  - [5. Documentation Tests (Doc Tests)](#5-documentation-tests-doc-tests)
  - [6. Crate-Level Documentation](#6-crate-level-documentation)
    - [Library Crates (`lib.rs`)](#library-crates-librs)
    - [Binary Crates (`main.rs`)](#binary-crates-mainrs)
  - [7. README Integration (Recommended)](#7-readme-integration-recommended)
  - [8. Generating Documentation](#8-generating-documentation)
    - [Generate Documentation Locally](#generate-documentation-locally)
    - [Generate and Open in Browser](#generate-and-open-in-browser)
    - [Include Private Items (Internal Review Only)](#include-private-items-internal-review-only)
  - [9. Workspace Documentation](#9-workspace-documentation)
  - [10. docs.rs Notes](#10-docsrs-notes)
  - [11. Tone and Formatting Rules](#11-tone-and-formatting-rules)
  - [12. Enforcement](#12-enforcement)
  - [13. Summary (Golden Rules)](#13-summary-golden-rules)

<!-- /TOC -->
 ---

## 1. Documentation Comment Style

### ✅ Use Line-Based Doc Comments (Required)

* **Item-level documentation**: `///`
* **Module / crate-level documentation**: `//!`

```rust
/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
//! This module provides utility functions.
```

### ❌ Do NOT Use Block-Based Doc Comments

The following styles are discouraged and must not be used in new code:

* `/** ... */`
* `/*! ... */`

Reasons:

* Lower readability
* More visual noise
* Inconsistent with Rust ecosystem conventions

---

## 2. What Must Be Documented

All **public items** MUST be documented:

* `pub fn`
* `pub struct`
* `pub enum`
* `pub trait`
* `pub impl` blocks (when they add public behavior)
* Public fields (when meaningful)

Private items may be documented at the author's discretion.

---

## 3. Documentation Examples by Item Type

### 3.1 Struct Documentation

```rust
/// Represents a user in the system.
///
/// # Fields
///
/// - `id`: Unique user identifier
/// - `name`: Display name
///
/// # Examples
///
/// ```
/// let user = User {
///     id: 1,
///     name: "Alice".to_string(),
/// };
///
/// assert_eq!(user.id, 1);
/// ```
pub struct User {
    pub id: u64,
    pub name: String,
}
```

---

### 3.2 Enum Documentation

```rust
/// Represents possible application errors.
///
/// # Variants
///
/// - `NotFound`: The requested item does not exist
/// - `Unauthorized`: Access is denied
///
/// # Examples
///
/// ```
/// let err = AppError::NotFound;
/// ```
pub enum AppError {
    /// The requested item was not found.
    NotFound,

    /// The operation is not permitted.
    Unauthorized,
}
```

---

### 3.3 Trait Documentation

```rust
/// Defines behavior for objects that can be persisted.
pub trait Persistable {
    /// Saves the object to storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn save(&self) -> Result<(), AppError>;
}
```

---

### 3.4 impl Block Documentation

Document an `impl` block **only if it introduces behavior
that is not obvious from individual methods.**

```rust
/// User-related business logic.
impl User {
    /// Returns the user's display name.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User { id: 1, name: "Alice".into() };
    /// assert_eq!(user.display_name(), "Alice");
    /// ```
    pub fn display_name(&self) -> &str {
        &self.name
    }
}
```

---

## 4. Required Sections for Public APIs

Every public function or method MUST include:

**`# Examples` (Required)**

* Must compile
* Must run
* Must reflect real-world usage

**Recommended sections when applicable:**

* `# Arguments`
* `# Returns`
* `# Errors`
* `# Panics`
* `# Safety` (for `unsafe` APIs)


### Example: Fully Documented Public Function

```rust
/// Reads a value from the buffer at the given index.
///
/// # Arguments
///
/// * `buffer` - A slice containing input data
/// * `index` - The position to read from
///
/// # Returns
///
/// Returns `Ok(value)` if the index is valid.
///
/// # Errors
///
/// Returns `AppError::NotFound` if `index` is out of bounds.
///
/// # Panics
///
/// Panics if `buffer` is empty.
///
/// # Examples
///
/// ```
/// let data = [10, 20, 30];
/// let value = read_checked(&data, 1).unwrap();
/// assert_eq!(value, 20);
/// ```
pub fn read_checked(buffer: &[i32], index: usize) -> Result<i32, AppError> {
    if buffer.is_empty() {
        panic!("buffer must not be empty");
    }

    buffer.get(index).copied().ok_or(AppError::NotFound)
}
```
**Note**
> Some examples include all sections (e.g., `# Panics`) as a template,
> even if not strictly necessary.

---

## 5. Documentation Tests (Doc Tests)

* All code blocks in documentation are treated as tests
* Must pass `cargo test`

Special annotations:

````rust
```no_run
```ignore
```should_panic
````

Use these only when strictly necessary.

---

## 6. Crate-Level Documentation

Each crate MUST include crate-level documentation.

### Library Crates (`lib.rs`)

* Describe purpose and scope
* Explain design principles
* Include at least one usage example
* Avoid CLI or I/O details

```rust
//! # my_lib
//!
//! Core business logic shared across binaries.
```

### Binary Crates (`main.rs`)

* Describe application purpose
* Provide usage examples
* Focus on CLI behavior, not APIs

```rust
//! # my_bin
//!
//! Command-line application built on top of `my_lib`.
```

---

## 7. README Integration (Recommended)

Library crates should synchronize `README.md` with rustdoc:

```rust
#![doc = include_str!("../README.md")]
```

This provides a single source of truth for documentation.

---

## 8. Generating Documentation

### Generate Documentation Locally

```bash
cargo doc
```

### Generate and Open in Browser

```bash
cargo doc --open
```

### Include Private Items (Internal Review Only)

```bash
cargo doc --document-private-items
```

---

## 9. Workspace Documentation

To generate documentation for the entire workspace:

```bash
cargo doc --workspace --open
```

---

## 10. docs.rs Notes

* `docs.rs` automatically runs `cargo doc`
* The crate-level documentation becomes the landing page
* Ensure all examples pass doc tests before publishing
* In a workspace, each crate is documented separately on docs.rs


---

## 11. Tone and Formatting Rules

* Use clear, professional English
* Prefer short, direct sentences
* Leave a blank line between sections
* Use Markdown supported by `rustdoc`

---

## 12. Enforcement

* Documentation quality is part of code review
* Public APIs without documentation will be rejected
* Failing doc tests must be fixed before merging
* CI may reject changes that introduce undocumented public APIs


---

## 13. Summary (Golden Rules)

* Use `///` and `//!` only
* Document all public items
* Every public API includes examples
* Documentation is executable and tested code


