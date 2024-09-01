use super::{
    BlockCell::BlockCell,
    Datacell::{CellType, DataCell},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BlockChildType {
    #[serde(rename = "~text~")]
    Text(TextCell),
    #[serde(rename = "~delimited~")]
    Delimited(DelimitedCell),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextCell {
    pub content: String,
    pub wrapped_with: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelimitedCell {
    pub open_delimeter: String,
    pub close_delimeter: String,
    pub terminal: String,
    pub display_type: DelimitedDisplayType,
    pub wrapped_with: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum DelimitedDisplayType {
    #[default]
    Default,
    INLINE,
    BLOCK,
}

pub trait BlockChild<T> {
    fn push_cell(parent: &mut BlockCell, s: T);
    fn insert_block(parent: &mut DataCell, s: T);
    fn add_block_at_first(add_to: &mut DataCell, parent_id: usize, block: &T);
    fn insert_text_to_first_block_child_text(add_to: &mut DataCell, parent_id: usize, text: &str);
}

impl BlockChild<BlockChildType> for BlockChildType {
    fn push_cell(parent: &mut BlockCell, block: BlockChildType) {
        parent.children.push(block)
    }

    fn insert_block(parent: &mut DataCell, block_child: BlockChildType) {
        if let CellType::Element(el) = &mut parent.cell_type {
            if let CellType::Block(block) = &mut el.children.first_mut().unwrap().cell_type {
                block.children.insert(0, block_child);
            }
        }
    }

    fn add_block_at_first(add_to: &mut DataCell, block_id: usize, block_child: &BlockChildType) {
        if add_to.id == block_id {
            Self::insert_block(add_to, block_child.to_owned());
            return;
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_block_at_first(x, block_id, block_child)),
            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_block_at_first(x, block_id, block_child)),
            _ => (),
        }
    }

    fn insert_text_to_first_block_child_text(
        add_to: &mut DataCell,
        block_id: usize,
        text_to_insert: &str,
    ) {
        if add_to.id == block_id {
            if let CellType::Element(el) = &mut add_to.cell_type {
                if let CellType::Block(block) = &mut el.children.first_mut().unwrap().cell_type {
                    if let BlockChildType::Text(text) = &mut block.children.first_mut().unwrap() {
                        text.content = text_to_insert.to_owned() + &text.content;
                    }
                }
            }
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el.children.iter_mut().for_each(|x| {
                Self::insert_text_to_first_block_child_text(x, block_id, text_to_insert)
            }),
            CellType::Root(ref mut el) => el.children.iter_mut().for_each(|x| {
                Self::insert_text_to_first_block_child_text(x, block_id, text_to_insert)
            }),
            _ => (),
        }
    }
}
