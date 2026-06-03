use crate::error::Result;
use lopdf::content::Content;
use lopdf::{Document as LopdfDoc, Object};
use pdf_agent_core::schema::bbox::BBox;
use pdf_agent_core::schema::char::Char;
use pdf_agent_core::schema::line::Line;
use pdf_agent_core::schema::span::Span;

pub struct NativeTextExtractor;

impl NativeTextExtractor {
    pub fn extract_page_text(doc: &LopdfDoc, page_num: u32, page_height: f64) -> Result<Vec<Line>> {
        let page_id = doc
            .page_iter()
            .nth((page_num - 1) as usize)
            .ok_or_else(|| crate::error::Error::PageOutOfBounds(page_num as usize))?;

        let content_data = doc.get_page_content(page_id)?;
        let content = Content::decode(&content_data)?;

        let mut lines: Vec<Line> = Vec::new();
        let mut current_spans: Vec<Span> = Vec::new();
        let mut current_chars: Vec<Char> = Vec::new();

        let mut x = 0.0;
        let mut y = 0.0;
        let mut font_name = "Helvetica".to_string();
        let mut font_size = 12.0;

        for operation in content.operations {
            match operation.operator.as_str() {
                "BT" => {
                    x = 0.0;
                    y = 0.0;
                }
                "ET" => {
                    if !current_chars.is_empty() {
                        let text: String = current_chars.iter().map(|c| c.value.as_str()).collect();
                        let mut span_bbox = BBox::new(x, y, x, y + font_size);
                        if let Some(first) = current_chars.first() {
                            span_bbox.x0 = first.bbox.x0;
                            span_bbox.y0 = first.bbox.y0;
                        }
                        if let Some(last) = current_chars.last() {
                            span_bbox.x1 = last.bbox.x1;
                            span_bbox.y1 = last.bbox.y1;
                        }
                        let span = Span::new(
                            text,
                            font_name.clone(),
                            font_size,
                            span_bbox,
                            std::mem::take(&mut current_chars),
                        );
                        current_spans.push(span);
                    }
                    if !current_spans.is_empty() {
                        let mut line_bbox = current_spans[0].bbox;
                        for s in &current_spans {
                            line_bbox = line_bbox.merge(&s.bbox);
                        }
                        lines.push(Line::new(std::mem::take(&mut current_spans), line_bbox));
                    }
                }
                "Tf" => {
                    if operation.operands.len() >= 2 {
                        if let Ok(name) = operation.operands[0].as_name_str() {
                            font_name = name.to_string();
                        }
                        if let Ok(size) = operation.operands[1].as_float() {
                            font_size = size as f64;
                        }
                    }
                }
                "Td" | "TD" => {
                    if operation.operands.len() >= 2 {
                        let tx = operation.operands[0].as_float().map(|f| f as f64).unwrap_or(0.0);
                        let ty = operation.operands[1].as_float().map(|f| f as f64).unwrap_or(0.0);
                        x += tx;
                        y += ty;
                    }
                }
                "Tm" => {
                    if operation.operands.len() >= 6 {
                        let e = operation.operands[4].as_float().map(|f| f as f64).unwrap_or(0.0);
                        let f = operation.operands[5].as_float().map(|f| f as f64).unwrap_or(0.0);
                        x = e;
                        y = f;
                    }
                }
                "Tj" => {
                    if operation.operands.len() >= 1 {
                        if let Ok(cow_str) = operation.operands[0].as_string() {
                            let text = cow_str.into_owned();
                            let mut cx = x;
                            for c in text.chars() {
                                let char_w = font_size * 0.6;
                                let char_bbox = BBox::new(cx, y, cx + char_w, y + font_size);
                                current_chars.push(Char::new(c.to_string(), char_bbox));
                                cx += char_w;
                            }
                            x = cx;
                        }
                    }
                }
                "TJ" => {
                    if operation.operands.len() >= 1 {
                        if let Ok(arr) = operation.operands[0].as_array() {
                            for obj in arr {
                                match obj {
                                    Object::String(bytes, _) => {
                                        let text = String::from_utf8_lossy(&bytes).into_owned();
                                        let mut cx = x;
                                        for c in text.chars() {
                                            let char_w = font_size * 0.6;
                                            let char_bbox = BBox::new(cx, y, cx + char_w, y + font_size);
                                            current_chars.push(Char::new(c.to_string(), char_bbox));
                                            cx += char_w;
                                        }
                                        x = cx;
                                    }
                                    Object::Integer(i) => {
                                        let offset = (*i as f64) / 1000.0 * font_size;
                                        x -= offset;
                                    }
                                    Object::Real(r) => {
                                        let offset = (*r as f64) / 1000.0 * font_size;
                                        x -= offset;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !current_chars.is_empty() {
            let text: String = current_chars.iter().map(|c| c.value.as_str()).collect();
            let span_bbox = BBox::new(x, y, x + text.len() as f64 * font_size * 0.6, y + font_size);
            let span = Span::new(
                text,
                font_name.clone(),
                font_size,
                span_bbox,
                std::mem::take(&mut current_chars),
            );
            current_spans.push(span);
        }
        if !current_spans.is_empty() {
            let mut line_bbox = current_spans[0].bbox;
            for s in &current_spans {
                line_bbox = line_bbox.merge(&s.bbox);
            }
            lines.push(Line::new(current_spans, line_bbox));
        }

        for line in &mut lines {
            line.bbox.y0 = page_height - line.bbox.y0;
            line.bbox.y1 = page_height - line.bbox.y1;
            if line.bbox.y0 > line.bbox.y1 {
                std::mem::swap(&mut line.bbox.y0, &mut line.bbox.y1);
            }

            for span in &mut line.spans {
                span.bbox.y0 = page_height - span.bbox.y0;
                span.bbox.y1 = page_height - span.bbox.y1;
                if span.bbox.y0 > span.bbox.y1 {
                    std::mem::swap(&mut span.bbox.y0, &mut span.bbox.y1);
                }

                for ch in &mut span.chars {
                    ch.bbox.y0 = page_height - ch.bbox.y0;
                    ch.bbox.y1 = page_height - ch.bbox.y1;
                    if ch.bbox.y0 > ch.bbox.y1 {
                        std::mem::swap(&mut ch.bbox.y0, &mut ch.bbox.y1);
                    }
                }
            }
        }

        lines.sort_by(|a, b| {
            a.bbox.y0
                .partial_cmp(&b.bbox.y0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut grouped_lines: Vec<Line> = Vec::new();
        for line in lines {
            if let Some(last_line) = grouped_lines.last_mut() {
                let y_diff = (last_line.bbox.y0 - line.bbox.y0).abs();
                if y_diff < 3.0 {
                    last_line.spans.extend(line.spans);
                    last_line.bbox = last_line.bbox.merge(&line.bbox);
                    last_line.spans.sort_by(|a, b| {
                        a.bbox.x0
                            .partial_cmp(&b.bbox.x0)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    continue;
                }
            }
            grouped_lines.push(line);
        }

        Ok(grouped_lines)
    }
}
