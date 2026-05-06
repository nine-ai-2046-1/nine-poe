# nine-poe

A minimal, secure Rust CLI tool for interacting with the [POE API](https://api.poe.com/v1/chat/completions).  
No external services, no black-box dependencies—just pure Rust with `reqwest`, `clap`, `serde`, and `toml`.

---

## Features

- **Simple CLI** — call any POE model with a single command
- **Session persistence** — continue conversations across multiple calls
- **Secure by default** — API key stored locally in `~/.nine-poe/config.toml`
- **Zero bloat** — only essential, auditable dependencies
- **Cross-platform** — works anywhere Rust compiles

---

## Quick Start

### 1. Build

```bash
cargo build --release
# Binary will be at target/release/nine-poe
```

### 2. Configure API Key

On first run, the config file is auto-created at `~/.nine-poe/config.toml`:

```toml
# Nine POE CLI Configuration
# Get your API key from: https://poe.com/api_key
NINE_POE_API_KEY = ""
```

Fill in your key:
```toml
NINE_POE_API_KEY = "your-actual-api-key-here"
```

### 3. Run

**Single message (no session):**
```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello, world!"
```

**With session (persists chat history):**
```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello!" --session "my_chat"
# Next call continues the conversation:
./target/release/nine-poe --model "Mistral-Small-4" --prompt "What did I just say?" --session "my_chat"
```

---

## CLI Usage

```
nine-poe [OPTIONS] --model <MODEL> --prompt <PROMPT>

Options:
  -m, --model <MODEL>       Model name (e.g. Mistral-Small-4, GPT-4, etc.)
  -p, --prompt <PROMPT>     User message / prompt
  -s, --session <SESSION>   Session name (optional, for chat history)
  -h, --help                Print help
  -V, --version             Print version
```

---

## How It Works

1. Reads `NINE_POE_API_KEY` from `~/.nine-poe/config.toml`
2. If `--session` is provided, loads chat history from `~/.nine-poe/sessions/{name}.json`
3. Sends a POST request to `https://api.poe.com/v1/chat/completions` with the messages
4. Prints the assistant's reply to stdout
5. If session was provided, appends both user and assistant messages to the session file

---

## Session Files

Session names are normalized (spaces → `_`, special characters removed).  
Files are stored as JSON:

```json
[
  { "role": "user", "content": "Hello!" },
  { "role": "assistant", "content": "Hi there! How can I help?" }
]
```

Location: `~/.nine-poe/sessions/{normalized_name}.json`

---

## Project Structure

```
nine-poe/
├── Cargo.toml
├── src/
│   ├── main.rs      # CLI entrypoint, argument parsing
│   ├── config.rs    # Config file management (~/.nine-poe/config.toml)
│   ├── session.rs   # Session persistence (JSON chat history)
│   └── api.rs       # POE API HTTP client
├── docs/
│   ├── README.md    # This file
│   ├── TECH.md      # Technical implementation details
│   └── CONTRIBUTION.md  # Contributor guide
└── references/      # Example curl scripts (for reference)
```

---

## Dependencies

| Crate    | Purpose                        |
|----------|--------------------------------|
| clap     | CLI argument parsing (derive)   |
| reqwest  | HTTP client (blocking, JSON)   |
| serde    | Serialization/deserialization  |
| serde_json | JSON handling              |
| toml     | Config file parsing            |
| dirs     | Cross-platform home directory  |

---

## License

MIT (or your preferred license)
