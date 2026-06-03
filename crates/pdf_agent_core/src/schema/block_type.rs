use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockType {
    Text,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    ListItem,
    Table,
    Equation,
    Image,
    Code,
    Unknown,
}

impl BlockType {
    pub fn is_heading(&self) -> bool {
        matches!(
            self,
            BlockType::Heading1
                | BlockType::Heading2
                | BlockType::Heading3
                | BlockType::Heading4
                | BlockType::Heading5
                | BlockType::Heading6
        )
    }
}
