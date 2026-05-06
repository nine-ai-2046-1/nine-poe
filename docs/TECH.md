# TECH.md — Technical Implementation Details

This document explains how `nine-poe` works under the hood, module by module.

---

## Architecture Overview

```
main.rs  ──→  config.rs   (load API key)
       │
       ├──→  session.rs  (load/save chat history)
       │
       └──→  api.rs      (HTTP POST to POE API)
```

---

## Module Details

### `src/config.rs` — Configuration Management

**Responsibility:** Ensure `~/.nine-poe/config.toml` exists and read the API key.

**Constants:**
```rust
const CONFIG_DIR: &str = ".nine-poe";
const CONFIG_FILE: &str = "config.toml";
const API_KEY_FIELD: &str = "NINE_POE_API_KEY";
```

**Key functions:**

| Function | Signature | Purpose |
|----------|-----------|---------|
| `get_config_path()` | `-> Result<PathBuf, ConfigError>` | Returns `~/.nine-poe/config.toml` path |
| `init_config()` | `-> Result<PathBuf, ConfigError>` | Creates dir + file if missing; writes sample TOML |
| `get_api_key()` | `-> Result<String, ConfigError>` | Parses TOML, returns trimmed key or `ApiKeyMissing` |

**Error handling:** `ConfigError` enum covers home dir missing, IO errors, TOML parse errors, and missing/empty API key.

**Config file format (TOML):**
```toml
NINE_POE_API_KEY = "your-key-here"
```

---

### `src/session.rs` — Session Persistence

**Responsibility:** Manage conversation history as JSON files under `~/.nine-poe/sessions/`.

**Message struct:**
```rust
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,    // "user" or "assistant"
    pub content: String,
}
```

**Session name normalization (`normalize_session_name`):**
1. Replace spaces with underscores
2. Keep only alphanumeric characters and underscores
3. Example: `"my chat session!"` → `"my_chat_session"`

**Key functions:**

| Function | Signature | Purpose |
|----------|-----------|---------|
| `get_sessions_dir()` | `-> Result<PathBuf, SessionError>` | Returns/creates `~/.nine-poe/sessions/` |
| `get_session_path(name)` | `-> Result<PathBuf, SessionError>` | Returns full path to `{name}.json` |
| `load_session(name)` | `-> Result<Vec<Message>, SessionError>` | Loads history; returns empty vec if file missing/empty |
| `save_session(name, messages)` | `-> Result<(), SessionError>` | Writes pretty-printed JSON to session file |

**Session file format (JSON):**
```json
[
  { "role": "user", "content": "Hello" },
  { "role": "assistant", "content": "Hi!" },
  { "role": "user", "content": "How are you?" }
]
```

---

### `src/api.rs` — POE API Integration

**Responsibility:** Send chat completion requests to the POE API and parse the response.

**API endpoint:**
```
POST https://api.poe.com/v1/chat/completions
```

**Request headers:**
```
Content-Type: application/json
Authorization: Bearer <NINE_POE_API_KEY>
anthropic-version: 2023-06-01
```

**Request body (serialized to JSON):**
```rust
struct Request {
    model: String,
    messages: Vec<Message>,  // Message is the same struct from session.rs
}
```

**Response parsing:**
```rust
struct Response {
    choices: Vec<Choice>,
}

struct Choice {
    message: ResponseMessage,
}

struct ResponseMessage {
    content: String,  // This is what we return!
}
```

**Key function:**
```rust
pub fn send_message(
    api_key: &str,
    model: &str,
    messages: Vec<Message>,
) -> Result<String, ApiError>
```

- Uses `reqwest::blocking::Client` (synchronous)
- Returns `choices[0].message.content` on success
- Returns `ApiError::NoChoices` if response has no choices

---

### `src/main.rs` — CLI Entrypoint

**Responsibility:** Parse CLI arguments, orchestrate the flow.

**CLI arguments (clap derive):**
```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long)] model: String,
    #[arg(short, long)] prompt: String,
    #[arg(short, long)] session: Option<String>,
}
```

**Runtime flow:**
```
1. Parse CLI args (clap)
2. Get API key via config::get_api_key()
   └─ If missing → print error, exit(1)
3. If session provided → load_session(session_name)
   └─ Returns Vec<Message> (history)
4. Append user message: Message { role: "user", content: prompt }
5. Call api::send_message(api_key, model, messages)
   └─ Print response to stdout
6. If session provided:
   └─ Append assistant message
   └─ save_session(session_name, messages)
```

---

## Data Flow Diagram

```
User Input (CLI)
    │
    ▼
[main.rs] Parse --model, --prompt, --session
    │
    ├─→ [config.rs] Read ~/.nine-poe/config.toml → API Key
    │
    ├─→ [session.rs] Load ~/.nine-poe/sessions/{name}.json → Vec<Message>
    │
    └─→ [api.rs] POST https://api.poe.com/v1/chat/completions
              Headers: Authorization: Bearer <key>
              Body: { "model": "...", "messages": [...] }
              Response: choices[0].message.content
                  │
                  ▼
              Print to stdout
                  │
                  ▼
        [session.rs] Save updated Vec<Message> to JSON file
```

---

## Dependencies & Rationale

| Dependency | Why it's used | Security note |
|------------|---------------|---------------|
| `reqwest` (blocking + json) | HTTP client for POE API | Synchronous; no async runtime needed for CLI |
| `clap` (derive) | CLI argument parsing | Standard Rust CLI library |
| `serde` + `serde_json` | Serialize/deserialize JSON | Only for session files and API request/response |
| `toml` | Parse config.toml | Only for `~/.nine-poe/config.toml` |
| `dirs` | Cross-platform home directory | No network calls; pure path logic |

**Security principle:** No dependency makes external API calls beyond the explicit POE API request. No telemetry, no analytics, no hidden network activity.

---

## Error Handling Strategy

| Layer | Error type | Behavior |
|-------|-----------|----------|
| Config | `ConfigError` | Print error, exit(1) |
| Session | `SessionError` | Print error, exit(1) |
| API | `ApiError` | Print error, exit(1) |
| All | — | All errors go to `eprintln!`, response to `println!` |

---

## File Locations Summary

| Purpose | Path |
|---------|------|
| Config file | `~/.nine-poe/config.toml` |
| Sessions directory | `~/.nine-poe/sessions/` |
| Session file | `~/.nine-poe/sessions/{normalized_name}.json` |
| POE API endpoint | `https://api.poe.com/v1/chat/completions` |
