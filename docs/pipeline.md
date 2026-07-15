# Pipeline

> This document is a summary reference. For the complete pipeline design, see [client-detailed-design.md](client-detailed-design.md) §4 and [api-contracts.md](api-contracts.md) §2-§4.

## Conversion Pipeline Overview

```text
Provider → Builder → Processors (serial chain) → Renderer → Output
```

### Stages

| Stage              | Component             | Description                            |
| ------------------ | --------------------- | -------------------------------------- |
| 1. LoadingPdf      | TextDocumentBuilder   | Read PDF, extract native text, build Document AST |
| 2. LayoutAnalysis  | LayoutPredictor       | Detect page layout blocks via ONNX     |
| 3. Ocr             | OcrPredictor          | Recognize text in scanned regions      |
| 4. RunningProcessors | Processor chain     | Table → Heading → List → LineMerge    |
| 5. Rendering       | MarkdownRenderer      | Convert AST to Markdown string         |

### OCR Decision Logic

```text
1. Try extract_native_text(page)
2. If lines.is_empty() → check ServiceRegistry for OcrService
3. If OcrService available → render_page(page, 150dpi) → recognize_text()
4. Use whichever lines are available to build Blocks
```

### Processor Execution Order

1. `TableProcessor` — detect and format table blocks
2. `HeadingProcessor` — infer heading levels from font metrics
3. `ListProcessor` — detect bullet/numbered list items
4. `LineMergeProcessor` — merge adjacent continuation lines

### Related Documents

- [client-detailed-design.md](client-detailed-design.md) §4 — Engine detailed design
- [api-contracts.md](api-contracts.md) §2.5-§2.7 — Builder/Processor/Renderer traits
- [api-contracts.md](api-contracts.md) §4.2 — PdfConverter API