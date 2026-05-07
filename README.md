# nine-poe 🚀

一個用 Rust 寫嘅超簡 CLI 工具，專登用嚟呼叫 POE API！  
冇任何黑盒依賴，淨係用 `reqwest`、`clap`、`serde` 同 `toml` 呢啲純正嘅 Rust crate。

---

## ✨ 功能特色

- **🎯 超簡 CLI** — 一條命令就叫到任何 POE 模型
- **💾 會話持久化** — 可以繼續之前嘅對話，唔使重複講
- **🔒 安全至上** — API Key 存喺本地 `~/.nine-poe/config.toml`
- **🪶 零嘥腫** — 淨係必需嘅、可以審查嘅依賴
- **🌍 跨平台** — 只要有 Rust 就可以編譯

---

## 🚀 快速開始

### 1️⃣ 編譯

```bash
cargo build --release
# 編譯好嘅執行檔會喺 target/release/nine-poe

# 建立 Global bin folder
mkdir -p ~/.nine-poe/bin/

# 放relase版本去好嘅地方
cp target/debug/nine-poe /path/you/like

# 更新 設定檔 (以下是Mac Terminal例子)
echo 'export PATH="$PATH:~/path/you/like/"' >> ~/.zshrc

# Activate 個change
source ~/.zshrc
```

### 2️⃣ 設定 API Key

第一次行嘅時候，設定檔會自動喺 `~/.nine-poe/config.toml` 生成：

```toml
# Nine POE CLI Configuration
# 去呢度攞你嘅 API Key：https://poe.com/api_key
NINE_POE_API_KEY = ""
```

填入你嘅 Key：
```toml
NINE_POE_API_KEY = "你嘅-api-key-喺呢度"
```

### 3️⃣ 開始用

**單次訊息（唔使 session）：**
```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello, world!"
```

**用 session（會記住對話歷史）：**
```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "Hello!" --session "my_chat"
# 下次再叫，會繼續之前嘅對話：
./target/release/nine-poe --model "Mistral-Small-4" --prompt "我頭先講咗啲咩？" --session "my_chat"
```

**📷 支援上傳圖片：**
```bash
./target/release/nine-poe --model "Mistral-Small-4" --prompt "描述呢張相" --file "/path/to/photo.jpg"
```

---

## 🎮 使用方法

```
nine-poe [OPTIONS] --model <MODEL> --prompt <PROMPT>

選項：
  -m, --model <MODEL>      模型名稱（例如：Mistral-Small-4, GPT-4 等等）
  -p, --prompt <PROMPT>    用戶訊息 / 提示詞
  -s, --session <SESSION>  會話名稱（可選，用嚟記住對話歷史）
  -f, --file <FILE>        本地文件路徑（圖片，可選）
  -h, --help               睇幫助
  -V, --version            睇版本
```

---

## 🔧 運作原理

1. 從 `~/.nine-poe/config.toml` 讀取 `NINE_POE_API_KEY`
2. 如果有俾 `--session`，就會從 `~/.nine-poe/sessions/{名稱}.json` 載入對話歷史
3. 發送 POST 請求去 `https://api.poe.com/v1/chat/completions`，帶住所有訊息
4. 將助手嘅回覆打印到 stdout
5. 如果有俾 session，就會將用戶同助手嘅訊息都存入 session 文件

---

## 💾 會話文件

會話名稱會被正規化（空格 → `_`，特殊字符會被移除）。  
文件以 JSON 格式儲存：

```json
[
  { "role": "user", "content": "Hello!" },
  { "role": "assistant", "content": "Hi there! How can I help?" }
]
```

位置：`~/.nine-poe/sessions/{正規化咗嘅名稱}.json`

---

## 📂 項目結構

```
nine-poe/
├── Cargo.toml
├── src/
│   ├── main.rs      # CLI 入口，處理參數
│   ├── config.rs    # 設定檔管理 (~/.nine-poe/config.toml)
│   ├── session.rs   # 會話持久化（JSON 對話歷史）
│   └── api.rs       # POE API HTTP 客戶端
├── docs/
│   ├── README.md    # 呢份文件（英文版）
│   ├── TECH.md      # 技術實現細節（英文版）
│   ├── CONTRIBUTION.md  # 貢獻者指南（英文版）
│   └── cantonese/   # 廣東話版文件
│       ├── README.md
│       ├── TECH.md
│       └── CONTRIBUTION.md
└── references/      # 參考 curl 腳本
```

---

## 📦 依賴

| Crate      | 用途                         |
|------------|------------------------------|
| clap       | CLI 參數解析（derive 模式）   |
| reqwest    | HTTP 客戶端（同步，JSON）    |
| serde      | 序列化 / 反序列化           |
| serde_json | JSON 處理                    |
| toml       | 設定檔解析                  |
| dirs       | 跨平台 home 目錄              |
| base64     | Base64 編碼（圖片用）        |
| mime_guess | MIME 類型檢測（圖片用）      |

---

## 📄 授權

MIT License
