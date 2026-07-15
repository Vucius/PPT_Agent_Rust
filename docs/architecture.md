# Architecture

> This document is a summary reference. For the complete architecture design, see [client-detailed-design.md](client-detailed-design.md) §1-§3.

## System Architecture

PPT_Agent_Rust follows a **Cargo Workspace + Layered Pipeline + iced UI Shell** architecture.

### Crate Dependency Graph

```text
pdf_agent_app (GUI)
  ├── pdf_agent_core (Pipeline Engine)
  │     ├──▷ pdf_agent_pdf (via DocumentProvider trait)
  │     ├──▷ pdf_agent_inference (via OcrProvider / LayoutProvider traits)
  │     └──▷ pdf_agent_llm (via LlmService trait)
  ├── pdf_agent_storage (SQLite + Keyring)
  └── pdf_agent_cli (Debugging)
```

### Dependency Constraints

- `core` defines traits; physical modules implement them
- Physical modules (`pdf`, `inference`, `llm`, `storage`) **never** depend on `app` or `core`
- `app` is the only crate that wires everything together

### Key Design Decisions

1. **Separation of Concerns** — GUI only handles interaction; `core` owns conversion flow
2. **Trait-Based DI** — `ServiceRegistry` uses `TypeId` + `Arc<dyn Any>` for type-safe dependency injection
3. **Async Pipeline** — All heavy work (PDF parsing, ONNX inference, HTTP) runs on Tokio workers; UI thread stays non-blocking
4. **Cooperative Cancellation** — `CancelToken` (AtomicBool) checked between pipeline stages

### Related Documents

- [client-detailed-design.md](client-detailed-design.md) — Full architecture, data models, thread model
- [api-contracts.md](api-contracts.md) — All trait signatures and API contracts
- [ui-interaction-spec.md](ui-interaction-spec.md) — UI/UX specification