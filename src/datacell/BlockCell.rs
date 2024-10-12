use super::{
    BlockChildType::TextCell,
    CellTrait::Cell,
    Datacell::{CellType, DataCell},
};
use serde::{Deserialize, Serialize};

#[inline]
fn is_false(v: &bool) -> bool {
    !(*v)
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockCell {
    pub children: Vec<TextCell>,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub has_counter_commands: bool,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub has_handle_insert: bool,
}

impl BlockCell {
    pub fn new() -> BlockCell {
        BlockCell {
            children: Vec::new(),
            ..Default::default()
        }
    }

    fn insert_cell(parent: &mut DataCell, id: usize, block: &BlockCell) {
        match parent.cell_type {
            CellType::Element(ref mut el) => {
                el.children.insert(0, Self::init_cell(id, parent.id, block))
            }
            _ => (),
        }
    }

    pub fn add_cell_at_first(
        add_to: &mut DataCell,
        parent_id: usize,
        id: usize,
        block: &BlockCell,
    ) {
        if add_to.id == parent_id {
            Self::insert_cell(add_to, id, block);
            return;
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell_at_first(x, parent_id, id, block)),
            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell_at_first(x, parent_id, id, block)),
            _ => (),
        }
    }
}

impl Cell<&BlockCell> for BlockCell {
    fn init_cell(id: usize, parent_id: usize, block: &BlockCell) -> DataCell {
        DataCell {
            id,
            parent_id,
            cell_type: CellType::Block(block.to_owned()),
        }
    }

    fn push_cell(parent: &mut DataCell, id: usize, block: &BlockCell) {
        match parent.cell_type {
            CellType::Element(ref mut el) => {
                el.children.push(Self::init_cell(id, parent.id, block))
            }
            _ => (),
        }
    }

    fn add_cell(add_to: &mut DataCell, parent_id: usize, id: usize, block: &BlockCell) {
        if add_to.id == parent_id {
            Self::push_cell(add_to, id, block);
            return;
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell(x, parent_id, id, block)),
            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell(x, parent_id, id, block)),
            _ => (),
        }
    }
}
