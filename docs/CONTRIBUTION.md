# CONTRIBUTION.md — Contributor Guide

Thank you for your interest in contributing to `nine-poe`!  
This guide explains how to get started, the coding conventions, and how to submit improvements.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- `cargo` (comes with Rust)
- A POE API key from https://poe.com/api_key

### Build & Run

```bash
# Clone the repo
git clone <repo-url>
cd nine-poe

# Build in debug mode (fast compile)
cargo build

# Build in release mode (optimized binary)
cargo build --release

# Run
./target/debug/nine-poe --model "Mistral-Small-4" --prompt "Hello"

# Or after release build:
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello"
```

### Test the CLI

1. Create `~/.nine-poe/config.toml` with your API key:
   ```toml
   NINE_POE_API_KEY = "your-key-here"
   ```

2. Test a single message:
   ```bash
   ./target/debug/nine-poe --model "Mistral-Small-4" --prompt "Hello!"
   ```

3. Test session persistence:
   ```bash
   ./target/debug/nine-poe --model "Mistral-Small-4" --prompt "First message" --session "test"
   ./target/debug/nine-poe --model "Mistral-Small-4" --prompt "Second message" --session "test"
   # Check the session file:
   cat ~/.nine-poe/sessions/test.json
   ```

---

## Project Structure & Module Map

```
src/
├── main.rs      # CLI entrypoint (clap derive, orchestration)
├── config.rs    # Config file: ~/.nine-poe/config.toml
├── session.rs   # Session persistence: JSON chat history
└── api.rs       # POE API HTTP client
```

**Before making changes, read:**
- `docs/TECH.md` for implementation details
- Each module's source file (they're short and well-commented by structure)

---

## Coding Conventions

### Rust Style
- Follow standard Rust conventions (`cargo fmt` before committing)
- Run `cargo clippy` and address warnings
- Keep functions focused and small
- No unnecessary comments—let the code be self-documenting

### Security-First Principle
This project prioritizes **security and transparency**:
- ✅ Use only the dependencies listed in `Cargo.toml`
- ✅ No dependencies that make external API calls beyond the explicit POE API request
- ✅ No telemetry, analytics, or hidden network activity
- ❌ Do NOT add crates that phone home, collect metrics, or call unknown services
- ❌ Do NOT suppress type errors with `as any`, `#[allow(...)]`, or `unwrap()` in production paths

### Error Handling
- Use the existing error enums (`ConfigError`, `SessionError`, `ApiError`)
- Print errors to `eprintln!`, responses to `println!`
- Exit with code `1` on errors

### Module Organization
When adding features:
- **Config-related** → `config.rs`
- **Session/chat-related** → `session.rs`
- **API/HTTP-related** → `api.rs`
- **New major feature** → consider a new module file

---

## How to Contribute

### Reporting Bugs

Open an issue with:
1. A clear description of the problem
2. Steps to reproduce
3. Expected vs. actual behavior
4. Your environment (OS, Rust version, nine-poe version)
5. The exact error message (if any)

**Bug report template:**
```
**Describe the bug**
A clear description of what went wrong.

**To Reproduce**
Steps to reproduce:
1. Run command: `nine-poe --model ... --prompt ...`
2. See error: ...

**Expected behavior**
What should have happened.

**Environment**
- OS: [e.g. macOS 14, Ubuntu 22.04]
- Rust version: [output of `rustc --version`]
- nine-poe version: [output of `nine-poe --version`]

**Additional context**
Any other relevant information.
```

### Suggesting Enhancements

Open an issue with:
1. The feature you'd like to see
2. Why it would be useful
3. A rough idea of how it could be implemented

### Pull Requests

1. **Fork** the repo and create a branch from `main`
2. **Code** your changes following the conventions above
3. **Test** thoroughly (build + manual CLI testing)
4. **Format** with `cargo fmt`
5. **Lint** with `cargo clippy`
6. **Document** if you're adding a new feature (update `docs/` if needed)
7. **Submit** the PR with a clear description

**PR checklist:**
- [ ] Code compiles with `cargo build`
- [ ] No new warnings (check `cargo build` output)
- [ ] Tested manually with the CLI
- [ ] Follows project coding conventions
- [ ] No unnecessary dependencies added
- [ ] Security principle maintained (no hidden network calls)

---

## Development Ideas (Good First Issues)

Here are some features that would be great contributions:

- [ ] **`--list-sessions` flag** — list all saved sessions in `~/.nine-poe/sessions/`
- [ ] **`--delete-session <name>`** — delete a specific session file
- [ ] **`--show-session <name>`** — pretty-print a session's chat history
- [ ] **Async support** — switch from `reqwest::blocking` to async (with `tokio`)
- [ ] **Streaming responses** — print chunks as they arrive (like ChatGPT streaming)
- [ ] **Better error messages** — replace Chinese error messages with configurable locale
- [ ] **Configurable API URL** — allow overriding `https://api.poe.com/v1/chat/completions`
- [ ] **Model validation** — warn if the model name doesn't look valid
- [ ] **Unit tests** — add `#[test]` functions for `normalize_session_name`, config init, session load/save
- [ ] **`--version` flag** — already exists via clap, but add build timestamp/commit hash

---

## Community & Questions

If you have questions or want to discuss a feature before implementing:
1. Open an issue with the "question" label
2. Describe what you're planning to do
3. We'll discuss the approach before you invest time coding

---

## License

By contributing, you agree that your contributions will be licensed under the project's MIT license.

Thank you for contributing! 🚀
