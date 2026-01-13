# Contributing Guide

Thank you for your interest in contributing to this workspace 🎉
This document explains **how to contribute**, **where rules are defined**,
and **what is expected during review**.

> **Important**
> This file is an entry point only.
> **It does not define coding style, documentation rules, or architecture.**

---

## 1. Where the Rules Live (Single Source of Truth)

To avoid duplication and inconsistencies, **each type of rule is defined in exactly one place**.

### 📐 Architecture Rules

All architectural decisions, crate responsibilities, and dependency constraints are defined in:

* **`docs/architecture.md`**

Do **NOT** re-define or reinterpret architecture rules in pull requests or reviews.
If a rule is unclear, propose a change **to `architecture.md` itself**.

---

### 📝 Documentation Rules

All Rust documentation conventions (rustdoc style, required sections, examples, etc.) are defined in:

* **`docs/docing-style.md`**

Public APIs **must** comply with this document.

---

## 2. Contribution Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes following the referenced rules
4. Ensure tests and documentation pass
5. Submit a pull request

---

## 3. Pull Request Checklist

Before opening a PR, ensure:

* [ ] Code follows the documented architecture
  → see `docs/architecture.md`
* [ ] Public APIs are documented
  → see `docs/docing-style.md`
* [ ] All tests pass (`cargo test`)
* [ ] Documentation builds (`cargo doc`)

---

## 4. Code Review Principles

Reviewers will evaluate:

* **Correctness**
* **Clarity**
* **Adherence to documented rules**

Reviews will **reference documentation**, not personal preferences.

If a reviewer requests a change based on architecture or documentation rules,
they must point to the relevant section in the referenced documents.

---

## 5. Proposing Rule Changes

If you believe a rule should change:

1. Open a PR that updates the relevant document:

   * `docs/architecture.md` **or**
   * `docs/docing-style.md`
2. Explain the motivation and impact
3. Get approval before applying the new rule to code

---

## 6. Summary

* CONTRIBUTING.md explains **process**, not **rules**
* Rules live in dedicated documents
* There is exactly **one authoritative definition per rule**
* When in doubt, update the source document, not this file

Happy hacking 🚀
