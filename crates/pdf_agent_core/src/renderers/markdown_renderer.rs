use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::renderers::traits::DocumentRenderer;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct MarkdownRenderer;

impl DocumentRenderer for MarkdownRenderer {
    type Output = String;

    fn render(&self, document: &Document, _ctx: &PipelineContext) -> Result<Self::Output> {
        let mut output = String::new();

        for page in &document.pages {
            output.push_str(&format!("<!-- Page {} -->\n\n", page.index + 1));

            let mut in_list = false;

            for block in &page.blocks {
                if block.block_type == BlockType::ListItem {
                    if !in_list {
                        in_list = true;
                    }
                    output.push_str(&format!("* {}\n", block.text.trim()));
                } else {
                    if in_list {
                        output.push_str("\n");
                        in_list = false;
                    }

                    match block.block_type {
                        BlockType::Heading1 => {
                            output.push_str(&format!("# {}\n\n", block.text.trim()));
                        }
                        BlockType::Heading2 => {
                            output.push_str(&format!("## {}\n\n", block.text.trim()));
                        }
                        BlockType::Heading3 => {
                            output.push_str(&format!("### {}\n\n", block.text.trim()));
                        }
                        BlockType::Heading4 => {
                            output.push_str(&format!("#### {}\n\n", block.text.trim()));
                        }
                        BlockType::Heading5 => {
                            output.push_str(&format!("##### {}\n\n", block.text.trim()));
                        }
                        BlockType::Heading6 => {
                            output.push_str(&format!("###### {}\n\n", block.text.trim()));
                        }
                        BlockType::Code => {
                            output.push_str(&format!("```\n{}\n```\n\n", block.text.trim()));
                        }
                        BlockType::Table => {
                            output.push_str(&format!("{}\n\n", block.text.trim()));
                        }
                        BlockType::Equation => {
                            output.push_str(&format!("$$\n{}\n$$\n\n", block.text.trim()));
                        }
                        BlockType::Image => {
                            output.push_str(&format!("![Image]({})\n\n", block.text.trim()));
                        }
                        _ => {
                            output.push_str(&format!("{}\n\n", block.text.trim()));
                        }
                    }
                }
            }

            if in_list {
                output.push_str("\n");
            }
            output.push_str("\n");
        }

        Ok(output)
    }
}
