# 0001 - Fly Rust Architecture and Build Decisions

Date: 2025-12-25  
Status: Accepted

## Context

I started Fly Rust with the goals of:

- Learning Rust and egui
- Exploring an MVVM architecture

## Decisions

- Rust: a programming language focused on speed, safety, and reliability.
- egui: a simple, fast, and portable immediate mode GUI library for Rust.
- MVVM: makes GUI code cleaner, more testable, and easier to maintain by separating View from Model.

---

## Consequences

- MVVM improves maintainability and testability by separating View from Model.
- Rust ensures performance, reliability, and memory safety.
- egui enables fast UI development and prototyping.
- Architecture supports cross-platform expansion and future feature additions.
- ADR documentation aids onboarding of new contributors.

The chosen architecture and tools ensure a maintainable, testable, and high-performance Rust GUI application,
while supporting rapid UI development and future scalability.


---

## Notes

* ADR stands for Architecture Decision Record.
* Future architectural changes should be documented in a new ADR, e.g., `0002-some-feature.md`.
* ADRs serve as a historical record and an onboarding guide for new contributors.

