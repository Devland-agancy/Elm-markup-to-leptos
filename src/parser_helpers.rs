use crate::{counter::counter_commands::CommandType, parser::*};
use leptos::{html::Data, server_fn::default};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde_json::*;
use syn::Block;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]

pub struct TagInfo {
    pub id: usize,
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "type")]
pub enum CellType {
    #[default]
    Default,
    Root(Root),
    #[serde(rename = "~element~")]
    Element(ElementCell),
    #[serde(rename = "~block~")]
    Block(BlockCell),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BlockChildType {
    #[serde(rename = "~text~")]
    Text(TextCell),
    #[serde(rename = "~delimited~")]
    Delimited(DelimitedCell),
    // #[serde(rename = "~counter~")]
    // Counter(CounterCell),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataCell {
    pub id: usize,
    pub parent_id: usize,
    #[serde(flatten)]
    pub cell_type: CellType,
}

impl DataCell {
    pub fn get_cell_by_id(cell: &mut DataCell, id: usize) -> Option<&mut DataCell> {
        if cell.id == id {
            return Some(cell);
        };

        match &mut cell.cell_type {
            CellType::Element(ref mut el) => el.children.iter_mut().find_map(|x| {
                if let Some(child) = Self::get_cell_by_id(x, id) {
                    Some(child)
                } else {
                    None
                }
            }),

            CellType::Root(ref mut el) => {
                let res = el.children.iter_mut().find_map(|x| {
                    if let Some(child) = Self::get_cell_by_id(x, id) {
                        Some(child)
                    } else {
                        None
                    }
                });
                res
            }
            _ => None,
        }
    }

    pub fn get_cell_by_id_immut(cell: &DataCell, id: usize) -> Option<&DataCell> {
        if cell.id == id {
            return Some(cell);
        };

        match &cell.cell_type {
            CellType::Element(el) => el.children.iter().find_map(|x| {
                if let Some(child) = Self::get_cell_by_id_immut(x, id) {
                    Some(child)
                } else {
                    None
                }
            }),

            CellType::Root(el) => {
                let res = el.children.iter().find_map(|x| {
                    if let Some(child) = Self::get_cell_by_id_immut(x, id) {
                        Some(child)
                    } else {
                        None
                    }
                });
                res
            }
            _ => None,
        }
    }

    pub fn into_el(&self) -> Option<&ElementCell> {
        if let CellType::Element(el) = &self.cell_type {
            return Some(el);
        }
        None
    }

    pub fn get_prev_sibling(&self, parent: &DataCell) -> Option<DataCell> {
        if let CellType::Element(parent) = &parent.cell_type {
            let index = parent.children.iter().position(|x| x.id == self.id)?;
            if index > 0 {
                return parent.children.iter().nth(index - 1).cloned();
            }
        }

        None
    }

    pub fn get_next_sibling(&self, parent: &DataCell) -> Option<DataCell> {
        if let CellType::Element(parent) = &parent.cell_type {
            let index = parent.children.iter().position(|x| x.id == self.id)?;
            return parent.children.iter().nth(index + 1).cloned();
        }

        None
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Root {
    pub children: Vec<DataCell>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ElementCell {
    #[serde(rename = "tag")]
    pub name: String,
    #[serde(rename = "attributes")]
    pub props: Vec<Prop>,
    pub children: Vec<DataCell>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Prop {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockCell {
    pub children: Vec<BlockChildType>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CounterCell {
    pub counter_name: String,
    pub command_type: CommandType,
    pub assign_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum DelimitedDisplayType {
    #[default]
    Default,
    INLINE,
    BLOCK,
}

pub trait Cell<T> {
    fn init_cell(id: usize, parent_id: usize, s: T) -> DataCell;
    fn push_cell(parent: &mut DataCell, id: usize, s: T);
    fn add_cell(add_to: &mut DataCell, parent_id: usize, id: usize, text: T);
}

impl ElementCell {
    pub fn push_attribute(&mut self, line: &str) {
        if let Some(prop_line) = line.split_once(" ") {
            if self
                .props
                .iter()
                .any(|x| x.key == prop_line.0 && x.value == prop_line.1)
            {
                return;
            }
            self.props.push(Prop {
                key: prop_line.0.to_string(),
                value: prop_line.1.to_string(),
            })
        }
    }

    pub fn add_attribute(tree: &mut DataCell, cell_id: usize, prop_line: &str) {
        if tree.id == cell_id {
            match &mut tree.cell_type {
                CellType::Element(ref mut el) => el.push_attribute(prop_line),
                _ => (),
            }
            return;
        }

        match &mut tree.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_attribute(x, cell_id, prop_line)),
            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_attribute(x, cell_id, prop_line)),
            _ => (),
        }
    }

    pub fn add_existing_cell(add_to: &mut DataCell, parent_id: usize, cell: &DataCell) {
        if add_to.id == parent_id {
            match add_to.cell_type {
                CellType::Element(ref mut el) => el.children.push(DataCell {
                    parent_id,
                    ..cell.to_owned()
                }),
                CellType::Root(ref mut el) => el.children.push(DataCell {
                    parent_id,
                    ..cell.to_owned()
                }),
                _ => (),
            }
            return;
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_existing_cell(x, parent_id, cell)),

            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_existing_cell(x, parent_id, cell)),
            _ => (),
        }
    }

    pub fn move_cell(
        tree: &mut DataCell,
        (cell_to_move_parent_id, cell_to_move_id): (usize, usize),
        move_to: usize,
    ) {
        if tree.id == cell_to_move_parent_id {
            if let CellType::Element(ref mut el) = tree.cell_type {
                // Get a mutable reference to `el`
                let (child_to_move, remaining_children): (Vec<_>, Vec<_>) = el
                    .children // Split children into two vecs
                    .iter()
                    .cloned() // Clone to avoid ownership issues
                    .partition(|child| child.id == cell_to_move_id);

                el.children = remaining_children;
                if let Some(child_to_move) = child_to_move.first() {
                    // Use if let to unwrap the child
                    Self::add_existing_cell(tree, move_to, child_to_move);
                }
            }

            return;
        }

        match &mut tree.cell_type {
            CellType::Element(ref mut el) => el.children.iter_mut().for_each(|x| {
                Self::move_cell(x, (cell_to_move_parent_id, cell_to_move_id), move_to)
            }),
            CellType::Root(ref mut el) => el.children.iter_mut().for_each(|x| {
                Self::move_cell(x, (cell_to_move_parent_id, cell_to_move_id), move_to)
            }),
            _ => (),
        }
    }

    pub fn move_children(tree: &mut DataCell, cell: usize, move_to: usize) {
        if tree.id == cell {
            let mut cloned_tree = tree.clone();
            if let CellType::Element(ref mut el) = &mut tree.cell_type {
                let move_to_cell = DataCell::get_cell_by_id(&mut cloned_tree, move_to);
                //let mut root = tree.to_owned();
                if let Some(move_to_cell) = move_to_cell {
                    if let CellType::Element(ref mut move_to_el) = move_to_cell.cell_type {
                        move_to_el.children = el
                            .children
                            .iter()
                            .cloned()
                            .partition(|child| child.id != move_to)
                            .0;
                        el.children.clear();
                        el.children.push(move_to_cell.to_owned())
                    }
                }
            }

            return;
        }

        match &mut tree.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::move_children(x, cell, move_to)),
            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::move_children(x, cell, move_to)),
            _ => (),
        }
    }
}

impl Cell<&str> for ElementCell {
    fn init_cell(id: usize, parent_id: usize, tag_name: &str) -> DataCell {
        DataCell {
            id,
            parent_id,
            cell_type: CellType::Element(ElementCell {
                name: tag_name.to_string(),
                ..Default::default()
            }),
        }
    }

    fn push_cell(parent: &mut DataCell, id: usize, tag_name: &str) {
        match parent.cell_type {
            CellType::Element(ref mut el) => {
                el.children.push(Self::init_cell(id, parent.id, tag_name))
            }
            CellType::Root(ref mut el) => {
                el.children.push(Self::init_cell(id, parent.id, tag_name))
            }
            _ => (),
        }
    }

    fn add_cell(add_to: &mut DataCell, parent_id: usize, id: usize, tag_name: &str) {
        if add_to.id == parent_id {
            Self::push_cell(add_to, id, tag_name);
            return;
        }

        match &mut add_to.cell_type {
            CellType::Element(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell(x, parent_id, id, tag_name)),

            CellType::Root(ref mut el) => el
                .children
                .iter_mut()
                .for_each(|x| Self::add_cell(x, parent_id, id, tag_name)),
            _ => (),
        }
    }
}

impl BlockCell {
    pub fn new() -> BlockCell {
        BlockCell {
            children: Vec::new(),
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

pub fn as_root(cell: &DataCell) -> Option<&Root> {
    match &cell.cell_type {
        CellType::Root(root) => Some(root),
        _ => None,
    }
}

pub fn tag_stack_pop(tag_stack: &mut Vec<TagInfo>, indent: &usize) {
    while let Some(last_tag_info) = tag_stack.last() {
        if *indent <= last_tag_info.indent {
            /* if last_tag_info.is_self_closing {
                output.push_str("/>\n");
            } else {
                output.push_str(&format!("</{}>\n", last_tag_info.name));
            } */
            tag_stack.pop();
        } else {
            break;
        }
    }
}
