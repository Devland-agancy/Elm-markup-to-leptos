use super::{
    BlockCell::BlockCell,
    Datacell::{CellType, DataCell},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextCell {
    pub content: String,
    pub wrapped_with: Option<String>,
}

impl TextCell {}

pub trait BlockChild<T> {
    fn push_cell(parent: &mut BlockCell, s: T);
    fn insert_block_child(parent: &mut DataCell, s: T);
    fn add_block_at_first(
        add_to: &mut DataCell,
        parent_id: usize,
        block: &T,
        block_options: Option<&BlockCell>,
    );
    fn insert_text_to_first_block_child_text(add_to: &mut DataCell, parent_id: usize, text: &str);
}

// impl BlockChild<BlockChildType> for BlockChildType {
//     fn push_cell(parent: &mut BlockCell, text: TextCell) {
//         parent.children.push(text)
//     }

//     fn insert_block_child(parent: &mut DataCell, block_child: BlockChildType) {
//         if let CellType::Element(el) = &mut parent.cell_type {
//             if let CellType::Block(block) = &mut el.children.first_mut().unwrap().cell_type {
//                 block.children.insert(0, block_child);
//             }
//         }
//     }

//     fn add_block_at_first(
//         add_to: &mut DataCell,
//         block_id: usize,
//         block_child: &BlockChildType,
//         block_options: Option<&BlockCell>,
//     ) {
//         if add_to.id == block_id {
//             if let CellType::Element(el) = &mut add_to.cell_type {
//                 if let CellType::Block(block) = &mut el.children.first_mut().unwrap().cell_type {
//                     block.children.insert(0, block_child.to_owned());
//                     if let Some(options) = block_options {
//                         block.has_counter_commands = options.has_counter_commands;
//                         block.has_handle_insert = options.has_handle_insert;
//                     }
//                 }
//             }
//             return;
//         }

//         match &mut add_to.cell_type {
//             CellType::Element(ref mut el) => el
//                 .children
//                 .iter_mut()
//                 .for_each(|x| Self::add_block_at_first(x, block_id, block_child, block_options)),
//             CellType::Root(ref mut el) => el
//                 .children
//                 .iter_mut()
//                 .for_each(|x| Self::add_block_at_first(x, block_id, block_child, block_options)),
//             _ => (),
//         }
//     }

//     fn insert_text_to_first_block_child_text(
//         add_to: &mut DataCell,
//         block_id: usize,
//         text_to_insert: &str,
//     ) {
//         if add_to.id == block_id {
//             if let CellType::Element(el) = &mut add_to.cell_type {
//                 if let CellType::Block(block) = &mut el.children.first_mut().unwrap().cell_type {
//                     if let BlockChildType::Text(text) = &mut block.children.first_mut().unwrap() {
//                         text.content = text_to_insert.to_owned() + &text.content;
//                     }
//                 }
//             }
//         }

//         match &mut add_to.cell_type {
//             CellType::Element(ref mut el) => el.children.iter_mut().for_each(|x| {
//                 Self::insert_text_to_first_block_child_text(x, block_id, text_to_insert)
//             }),
//             CellType::Root(ref mut el) => el.children.iter_mut().for_each(|x| {
//                 Self::insert_text_to_first_block_child_text(x, block_id, text_to_insert)
//             }),
//             _ => (),
//         }
//     }
// }
