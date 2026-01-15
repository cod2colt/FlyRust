# Change Log

---

### v0.0.1 (2026-01-01)

#### INTRODUCTION

- **FlyRust** is a **game** developed in **Rust** using **egui**, following the **MVVM architecture**.

- **Fly with Rust** is a **Rust** project named ***FlyRust***, created as a hands-on practice for learning Rust.
The idea is to learn Rust by building and flying with it, hence the name Fly Rust.

#### FEATURES, ENHANCEMENTS, & BUGFIXES

- Create the workspace for the flyrust based on Rust and egui
- Build with MVVM architecture.
- i18n support: English, 繁體中文, 简体中文.
- macOS app packaging
- Add `macapp_build.sh` script:
  - Build release code and package into `.app`.
  - Sync `Contents/Resources/assets` data with macOS app package.
  - Patch paths to ensure all asset files reference absolute paths, not relative.
  - Replace `assets/` with `Workspace/Contents/Resources/assets/` to keep consistent in debug, release, and macOS app package.
- Add `what_panic()` to log panic information. Check logs via `cat /tmp/what_panic.log`.

