# Project and User Background

> Do not use plan mode unless user explicitly mentioned

## Purpose

This repository contains a high-performance, low-memory PDF document parsing and multi-agent pipeline based on the architecture of Marker, written in Rust. It features a layered pipeline (`Providers -> Builders -> Processors -> Renderers -> Extractors -> Services`) and uses an `iced` GUI desktop shell and a command-line interface (CLI) to process documents.

## Ground Truth

- Rust packages and workspaces are managed in the `crates/` directory.
- Root `Cargo.toml` defines the workspace members:
  - `crates/pdf_agent_app`: iced desktop GUI.
  - `crates/pdf_agent_core`: Core document transformation pipeline (Provider, Builder, Processor, Renderer, Extractor traits and runtime engine).
  - `crates/pdf_agent_pdf`: PDF layout coordinates, native text, and high/low-DPI page rendering backend (via `pdfium-render` / `lopdf`).
  - `crates/pdf_agent_inference`: Local machine learning inference using ONNX Runtime (`ort` crate) for Layout and OCR models.
  - `crates/pdf_agent_llm`: LLM API adapters (Gemini, Claude, OpenAI, Ollama) and prompt processing.
  - `crates/pdf_agent_storage`: Local SQLite database and OS keyring manager integration for API key and task quota history storage.
  - `crates/pdf_agent_cli`: Command line debugging interface.
- Local model files are described or stored in `models/`.
- Database schema migrations are in `migrations/`.

## Core Philosophy

### 1. Good Taste First
- Keep functions short, focused, and strongly typed.
- Prefer elegant separation of concerns. Do not mix GUI logic with PDF parsing, ORT inference, or database code.
- Avoid introducing special-case branches and nesting. Prefer structuring models so edge cases map to ordinary cases.

### 2. Pragmatism Over Theory
- Solve the real problem in this repository, not a hypothetical one.
- Keep the architecture as thin as possible to do the job. 
- Use the Rust type system to enforce constraints rather than complex runtime reflection.

### 3. Simplicity As A Constraint
- Limit nesting to 3 levels.
- Keep dependencies minimal to ensure fast compilation and easy cross-compilation.

## Code Style Rules

1. Avoid excessive error masking. Propagation (`?`) is preferred, and panics/unwraps are forbidden in core/library crates unless mathematically proven safe.
2. Implement custom errors using crates like `thiserror` in each crate.
3. Write technical documentation and comments in English.
4. Prefer modern Rust tooling:
   - Use workspace dependencies where appropriate.
   - Utilize current library APIs (e.g. `serde` for serialization, `tokio` for async).
5. Prefer fewer dependencies and less code.

## Communication Rules

- Think in English, reply to the user in Chinese.
- Be direct and concise. If code or architectural suggestion is sub-optimal, explain why technically.
- Maintain a collaborative and constructive tone.

## High-Level Architecture

### `pdf_agent_app` (iced UI)
- `src/main.rs`: Application entrypoint.
- `src/app.rs`: Main update, view, and subscription loop.
- `src/message.rs`: UI event messages.
- `src/screens/`: Views for home, import, convert, result, history, and settings.
- `src/components/`: Reusable widgets (dropzones, progress bars, sidebars).

### `pdf_agent_core` (Core Pipeline)
- `src/schema/`: The internal document AST representation (`Document -> Page -> Block -> Line -> Span -> Char`).
- `src/pipeline/`: `Converter` orchestrator that wires up provider, builders, processors, and renderers.
- `src/context/`: `PipelineContext` and `ServiceRegistry` implementing type-safe dependency injection.
- `src/processors/`: Markdown cleanup, list formatting, table reconstruction, and LLM text rewriting.

### `pdf_agent_pdf` (PDF Engine)
- `src/pdf_provider.rs`: Implements `DocumentProvider` trait using `pdfium-render` and `lopdf`.
- `src/coordinates.rs`: Coordinates conversion helper (PDF bottom-left coordinate space vs. Image top-left space).

### `pdf_agent_inference` (ONNX Runtime)
- `src/predictors/`: Layout detection, line detection, and OCR predictors.
- Uses `ort` wrapper to run local ONNX models on CPU or GPU (DirectML/CoreML).

### `pdf_agent_llm` (LLM Services)
- `src/providers/`: OpenAI, Gemini, Anthropic, and Ollama clients.
- Implements rate limiting and token bucket logic for API usage.

### `pdf_agent_storage` (Local Storage)
- `src/db.rs`: SQLite database integration.
- `src/keyring_store.rs`: Secure storage of LLM API keys via OS keychain (`keyring-rs`).

### `pdf_agent_cli` (CLI tool)
- Simple executable to debug transformation pipelines without running the GUI.

## Working Rules For Agents

- Identify the correct crate that owns a specific behavior before editing.
- Ensure proper dependency separation: `core` must never depend on `app`; helper crates must remain independent.
- Keep changes scoped. Run `cargo check` and unit tests after making modifications.
- Do not add new external crates to `Cargo.toml` without verifying they are necessary and lightweight.

## Known Sharp Edges

- **Coordinate Systems**: PDF coordinate system origin is at the bottom-left, while images and screens use top-left. Coordinate conversions must be handled carefully in `crates/pdf_agent_pdf/src/coordinates.rs`.
- **Model Lazy Loading**: Recognition/OCR models are heavy (hundreds of MBs). They must be lazily loaded in `pdf_agent_inference` only when native text extraction fails, and dropped promptly to save memory.
- **API Key Storage**: Always store API keys in the system keyring (`keyring-rs`), never in the plain SQLite database.
- **Double-Channel DPI**: Use 96 DPI for Layout/Detection bounding boxes and 192 DPI for high-precision OCR crops.

## File Map

- `Cargo.toml`: Cargo workspace configuration.
- `README.md`: User-facing overview.
- `execution_plan.md`: Technical architecture and UI/UX execution plan.
- `marker_analysis.md`: In-depth analysis of the Marker Python architecture.
- `AGENTS.md`: Agent workspace context and rules (this file).

## Preferred Change Strategy

1. Identify which crate (app, core, pdf, inference, llm, storage, cli) owns the feature or bug.
2. Edit the smallest relevant surface.
3. Validate by running `cargo check` and specific crate tests.
