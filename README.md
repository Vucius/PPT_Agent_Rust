# PPT_Agent_Rust

A high-performance, low-memory PDF document parsing and conversion desktop application written in Rust. Built on a layered pipeline architecture inspired by [Marker](https://github.com/VikParuchuri/marker), it converts PDF files into clean Markdown/JSON output with optional OCR and LLM-powered correction.

## Features

- **PDF → Markdown Conversion** — Extract native text from PDF files and produce well-formatted Markdown
- **Side-by-Side Preview** — Compare original PDF rendering with converted Markdown output
- **OCR Fallback** — Automatically detect scanned pages and fall back to ONNX-based OCR recognition
- **Layout Detection** — ONNX-based page layout analysis for tables, headings, equations, and images
- **LLM-Powered Correction** — Select any block and submit natural-language feedback; the app sends context to an LLM and generates a correction patch
- **Diff Preview** — Review LLM corrections with a before/after diff view before applying
- **Version History** — Full undo/redo with snapshot-based version management
- **Secure API Key Storage** — API keys stored in the OS native keyring (Windows Credential Manager / macOS Keychain)
- **Token Quota Management** — Daily token usage tracking with configurable limits

## Architecture

The project uses a **Cargo Workspace** with 7 crates following a strict dependency hierarchy:

```text
pdf_agent_app       — iced desktop GUI shell
pdf_agent_core      — Core document conversion pipeline engine
pdf_agent_pdf       — PDF reading, rendering, and native text extraction
pdf_agent_inference  — Local ONNX Runtime inference (Layout + OCR)
pdf_agent_llm       — LLM API adapters (OpenAI, Gemini, Anthropic, Ollama)
pdf_agent_storage   — SQLite database + OS keyring integration
pdf_agent_cli       — Command-line debugging interface
```

**Core Design Principle**: *"PDF conversion engine + desktop shell"*, not *"desktop project with PDF features"*.

```text
iced   → interaction
core   → pipeline flow
pdf    → file I/O
inference → models
llm    → enhancement
storage → state
cli    → debugging
```

## Prerequisites

- **Rust** ≥ 1.75 (edition 2021)
- **PDFium** dynamic library — required by `pdfium-render` for PDF rendering
  - Windows: `pdfium.dll` in PATH or alongside the executable
  - macOS: `libpdfium.dylib`
  - Linux: `libpdfium.so`
- **ONNX Runtime** (optional) — required for real Layout/OCR model inference

## Quick Start

### Build

```bash
# Clone the repository
git clone <repo-url>
cd PPT_Agent_Rust

# Build all workspace members
cargo build

# Build in release mode
cargo build --release
```

### Run the Desktop App

```bash
cargo run -p pdf_agent_app
```

### Run the CLI

```bash
cargo run -p pdf_agent_cli -- --input sample.pdf --output output.md
```

### Run Tests

```bash
cargo test --workspace
```

## Project Structure

```text
PPT_Agent_Rust/
├── Cargo.toml                  # Workspace configuration
├── README.md                   # This file
├── LICENSE                     # Apache 2.0
├── execution_plan.md           # Technical execution plan (1187 lines)
├── marker_analysis.md          # Marker Python architecture analysis
├── AGENTS.md                   # Agent workspace context and rules
│
├── docs/
│   ├── roadmap.md              # Development roadmap with phases
│   ├── ui-interaction-spec.md  # UI/UX interaction specification
│   ├── client-detailed-design.md # Client detailed design document
│   ├── api-contracts.md        # Interface definitions & API contracts
│   ├── architecture.md         # Architecture overview
│   ├── pipeline.md             # Pipeline details
│   └── data-model.md           # Data model schema
│
├── crates/
│   ├── pdf_agent_app/          # iced desktop GUI
│   ├── pdf_agent_core/         # Core pipeline engine
│   ├── pdf_agent_pdf/          # PDF physical layer
│   ├── pdf_agent_inference/    # ONNX inference
│   ├── pdf_agent_llm/          # LLM service adapters
│   ├── pdf_agent_storage/      # SQLite + Keyring
│   └── pdf_agent_cli/          # CLI tool
│
├── models/                     # Local ONNX model files
├── migrations/                 # SQLite schema migrations
├── assets/                     # Icons, fonts, samples
└── tests/                      # Integration test fixtures
```

## Document Pipeline

The conversion pipeline follows a `Provider → Builder → Processor → Renderer` flow:

```text
1. PdfProvider reads PDF → extracts native text + renders page images
2. TextDocumentBuilder constructs Document AST (Document → Page → Block → Line → Span → Char)
3. Processors refine the AST:
   ├── TableProcessor    — detect and format tables
   ├── HeadingProcessor  — infer heading levels from font size
   ├── ListProcessor     — detect and merge list items
   └── LineMergeProcessor — merge adjacent text lines
4. MarkdownRenderer produces final Markdown output
```

## Configuration

### LLM Settings

The app supports multiple LLM providers. Configure via the Settings tab:

| Setting       | Default                        | Description                  |
| ------------- | ------------------------------ | ---------------------------- |
| Provider      | `mock`                         | mock / openai / gemini / ollama |
| Model         | `gpt-4o-mini`                  | Model name                   |
| Base URL      | `https://api.openai.com/v1`    | API endpoint                 |
| Daily Limit   | `50000`                        | Max tokens per day           |

### OCR Mode

| Mode     | Behavior                                              |
| -------- | ----------------------------------------------------- |
| `auto`   | Try native text first; fall back to OCR if empty      |
| `always` | Force OCR on all pages                                |
| `never`  | Native text only; skip OCR entirely                   |

## Key Dependencies

| Crate           | Purpose                          |
| --------------- | -------------------------------- |
| `iced`          | Cross-platform GUI framework     |
| `lopdf`         | PDF logical structure parsing    |
| `pdfium-render` | PDF page rendering (via PDFium)  |
| `tokio`         | Async runtime                    |
| `serde`         | Serialization                    |
| `rusqlite`      | SQLite database                  |
| `keyring`       | OS keyring integration           |
| `reqwest`       | HTTP client for LLM APIs        |
| `similar`       | Text diff computation            |
| `rfd`           | Native file dialogs              |
| `async-trait`   | Async trait support              |
| `thiserror`     | Error type derivation            |

## Development Status

| Phase | Description                          | Status      |
| ----- | ------------------------------------ | ----------- |
| 1     | PDF native text → Markdown           | ✅ Complete |
| 2     | Page rendering + side-by-side preview | ✅ Complete |
| 3     | Document AST + core processors       | ✅ Complete |
| 4     | OCR inference + Layout detection     | ✅ Complete |
| 5     | LLM patch + Diff + version history   | ✅ Complete |
| A     | App layer componentization refactor  | 🔄 Next    |
| B     | Real Diff rendering + file export    | ☐ Planned  |
| C     | Processor & Renderer expansion       | ☐ Planned  |
| D     | LLM provider expansion + tests       | ☐ Planned  |
| E     | Performance polish + packaging       | ☐ Planned  |

See [docs/roadmap.md](docs/roadmap.md) for the full development roadmap.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
