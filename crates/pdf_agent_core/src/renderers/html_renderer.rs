use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::renderers::traits::DocumentRenderer;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct HtmlRenderer;

impl DocumentRenderer for HtmlRenderer {
    type Output = String;

    fn render(&self, document: &Document, _ctx: &PipelineContext) -> Result<Self::Output> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        
        let title = html_escape(&document.file_name);
        html.push_str(&format!("  <title>{}</title>\n", title));
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: sans-serif; line-height: 1.6; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("    table { border-collapse: collapse; width: 100%; margin-bottom: 1em; }\n");
        html.push_str("    th, td { border: 1px solid #ddd; padding: 8px; }\n");
        html.push_str("    img { max-width: 100%; height: auto; }\n");
        html.push_str("    pre, code { background-color: #f4f4f4; padding: 2px 4px; border-radius: 4px; }\n");
        html.push_str("    .page-break { border-bottom: 2px dashed #ccc; margin: 40px 0; text-align: center; color: #999; }\n");
        html.push_str("  </style>\n</head>\n<body>\n");

        for page in &document.pages {
            html.push_str(&format!("<div id=\"page-{}\" class=\"page\">\n", page.index));
            
            for block in &page.blocks {
                if block.text.trim().is_empty() {
                    continue;
                }

                let escaped_text = html_escape(&block.text);
                
                match block.block_type {
                    BlockType::Heading1 => html.push_str(&format!("  <h1>{}</h1>\n", escaped_text)),
                    BlockType::Heading2 => html.push_str(&format!("  <h2>{}</h2>\n", escaped_text)),
                    BlockType::Heading3 => html.push_str(&format!("  <h3>{}</h3>\n", escaped_text)),
                    BlockType::Heading4 => html.push_str(&format!("  <h4>{}</h4>\n", escaped_text)),
                    BlockType::Heading5 => html.push_str(&format!("  <h5>{}</h5>\n", escaped_text)),
                    BlockType::Heading6 => html.push_str(&format!("  <h6>{}</h6>\n", escaped_text)),
                    BlockType::ListItem => {
                        // A very basic representation. Ideally we'd wrap adjacent ListItems in <ul>
                        html.push_str(&format!("  <ul><li>{}</li></ul>\n", escaped_text))
                    }
                    BlockType::Table => {
                        html.push_str("  <table>\n");
                        for row in escaped_text.split('\n') {
                            if row.trim().is_empty() { continue; }
                            html.push_str("    <tr>\n");
                            // Simple split by typical markdown table separators like '|' if applicable, 
                            // or just dump it as preformatted text if it's raw text.
                            // Assuming raw text separated by spaces for now as a fallback:
                            html.push_str(&format!("      <td><pre>{}</pre></td>\n", row));
                            html.push_str("    </tr>\n");
                        }
                        html.push_str("  </table>\n");
                    }
                    BlockType::Equation => {
                        // Wrap in div for block equations
                        html.push_str(&format!("  <div class=\"equation\">\n    {}\n  </div>\n", escaped_text))
                    }
                    BlockType::Code => html.push_str(&format!("  <pre><code>{}</code></pre>\n", escaped_text)),
                    BlockType::Image => html.push_str(&format!("  <img alt=\"{}\" />\n", escaped_text)),
                    BlockType::Text | BlockType::Unknown => html.push_str(&format!("  <p>{}</p>\n", escaped_text)),
                }
            }
            
            html.push_str("</div>\n");
            html.push_str(&format!("<div class=\"page-break\">--- Page {} ---</div>\n", page.index + 1));
        }

        html.push_str("</body>\n</html>");
        Ok(html)
    }
}

fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{bbox::BBox, block::Block, page::Page};
    use crate::context::ServiceRegistry;

    fn make_block(id: &str, text: &str, btype: BlockType) -> Block {
        Block::new(id.to_string(), BBox::new(0.0, 0.0, 10.0, 10.0), btype, text.to_string(), vec![])
    }

    #[test]
    fn test_html_renderer() {
        let b1 = make_block("1", "Title", BlockType::Heading1);
        let b2 = make_block("2", "Some <text> & symbols", BlockType::Text);
        let page1 = Page::new(0, 800.0, 600.0, vec![b1, b2]);

        let doc = Document::new("test.pdf".to_string(), vec![page1]);
        let ctx = PipelineContext::new(crate::config::PipelineConfig::default(), ServiceRegistry::new());

        let renderer = HtmlRenderer;
        let html = renderer.render(&doc, &ctx).unwrap();

        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<p>Some &lt;text&gt; &amp; symbols</p>"));
        assert!(html.contains("<!DOCTYPE html>"));
    }
}
