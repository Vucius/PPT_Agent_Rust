# Data Model

> This document is a summary reference. For the complete data model definitions, see [client-detailed-design.md](client-detailed-design.md) §2 and [api-contracts.md](api-contracts.md) §3.

## Document Tree (AST)

```text
Document
├── file_name: String
├── metadata: HashMap<String, String>
└── pages: Vec<Page>
    └── Page
        ├── index: usize
        ├── width: f64
        ├── height: f64
        └── blocks: Vec<Block>
            └── Block
                ├── id: String          (format: "p{page}_b{idx}")
                ├── bbox: BBox          { x0, y0, x1, y1 }
                ├── block_type: BlockType
                ├── text: String
                └── lines: Vec<Line>
                    └── Line
                        ├── bbox: BBox
                        └── spans: Vec<Span>
                            └── Span
                                ├── text: String
                                ├── font_name: String
                                ├── font_size: f64
                                ├── bbox: BBox
                                └── chars: Vec<Char>
                                    └── Char
                                        ├── value: String
                                        └── bbox: BBox
```

## BlockType Enum

```text
Text | Heading1..Heading6 | ListItem | Table | Equation | Image | Code | Unknown
```

## BBox (Bounding Box)

```text
(x0, y0) ─────────────────── (x1, y0)
   │                              │
   │        content area          │
   │                              │
(x0, y1) ─────────────────── (x1, y1)

Coordinate system: origin at top-left (image/screen space)
PDF uses bottom-left origin → coordinates.rs handles conversion
```

## Related Documents

- [client-detailed-design.md](client-detailed-design.md) §2 — Full class diagram and ID naming rules
- [api-contracts.md](api-contracts.md) §3 — Data contract field-level documentation
- [api-contracts.md](api-contracts.md) §4.4 — Document API (find_block_with_context, update_block_text)