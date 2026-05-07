# nine-poe 🚀

A super simple CLI tool written in Rust 🦀, specifically designed to call the POE API!  
No black-box dependencies—only pure Rust crates like `reqwest`📡, `clap`⚙️, `serde`🔧, and `toml`📁 are used.

---

## ✨ Features

- **🎯 Super simple CLI** — Call any POE model with just one command
- **💾 Session persistence** — Continue previous conversations without repeating yourself
- **🔒 Security first** — API Key stored locally in `~/.nine-poe/config.toml`
- **🪶 Minimal bloat** — Only essential, audit-able dependencies
- **🌍 Cross-platform** — Can compile anywhere Rust runs

---

## 🚀 Quick Start

### 1️⃣ Build

```bash
cargo build --release
# The compiled executable will be at target/release/nine-poe

# Create global bin folder
mkdir -p ~/.nine-poe/bin/

# Copy the release file to a preferred location
cp target/debug/nine-poe /path/you/like

# Update PATH (example for macOS Terminal)
echo 'export PATH="$PATH:~/path/you/like/"' >> ~/.zshrc

# Activate the change
source ~/.zshrc
```

### 2️⃣ Configure API Key

The first run will auto-generate a config file at `~/.nine-poe/config.toml`:

```toml
# Nine POE CLI Configuration
# Get your API Key here: https://poe.com/api_key
NINE_POE_API_KEY = ""
```

Fill in your key:

```toml
NINE_POE_API_KEY = "your-api-key-here"
```

### 3️⃣ Start Using

**Single message (no session needed):**

```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello, world!"
```

**With session (remembers conversation history):**

```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello!" --session "my_chat"
# Next call continues the conversation:
./target/release/nine-poe --model "Mistral-Small-4" --prompt "What did I just say?" --session "my_chat"
```

**📷 Supports image upload:**

```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Describe this photo" --file "/path/to/photo.jpg"
```

---

## 🎮 Usage

```
nine-poe [OPTIONS] --model <MODEL> --prompt <PROMPT>

Options:
  -m, --model <MODEL>       Model name (e.g., Mistral-Small-4, GPT-4, etc.)
  -p, --prompt <PROMPT>     User message / prompt
  -s, --session <SESSION>   Session name (optional, to remember conversation history)
  -f, --file <FILE>         Local file path (image, optional)
  -h, --help                Show help
  -V, --version             Show version
```

---

## 🔧 How It Works

1. Reads `NINE_POE_API_KEY` from `~/.nine-poe/config.toml` 🔑  
2. Loads conversation history from `~/.nine-poe/sessions/{name}.json` if `--session` is used 💬  
3. Sends a POST request to `https://api.poe.com/v1/chat/completions` with all messages 📤  
4. Prints the assistant’s reply to stdout 🖥️  
5. Saves user and assistant messages to the session file if a session is used 💾

---

## 💾 Session Files

Session names get normalized (spaces → `_`, special characters removed).  
Stored in JSON format:

```json
[
  { "role": "user", "content": "Hello!" },
  { "role": "assistant", "content": "Hi there! How can I help?" }
]
```

Location: `~/.nine-poe/sessions/{normalized_name}.json`

---

## 📂 Project Structure

```
nine-poe/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI entry, argument handling
│   ├── config.rs      # Config management (~/.nine-poe/config.toml)
│   ├── session.rs     # Session persistence (JSON chat history)
│   └── api.rs         # POE API HTTP client
├── docs/
│   ├── README.md      # This document (English)
│   ├── TECH.md        # Technical details (English)
│   ├── CONTRIBUTION.md# Contributor guide (English)
│   └── cantonese/     # Cantonese docs
│       ├── README.md
│       ├── TECH.md
│       └── CONTRIBUTION.md
└── references/        # Reference curl scripts
```

---

## 📦 Dependencies

| Crate      | Purpose                       |
|------------|-------------------------------|
| clap       | CLI argument parsing (derive) |
| reqwest    | HTTP client (sync, JSON)       |
| serde      | Serialization/deserialization  |
| serde_json | JSON handling                  |
| toml       | Config file parsing            |
| dirs       | Cross-platform home directory  |
| base64     | Base64 encoding (for images)   |
| mime_guess | MIME type detection (images)   |

---

## 📄 License

MIT License
