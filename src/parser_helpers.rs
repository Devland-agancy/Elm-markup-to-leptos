use crate::parser::*;
use leptos::html::Data;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde_json::*;
use syn::Block;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]

pub struct TagInfo {
    pub id: u32,
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
    Element(ElementCell),
    Block(BlockCell),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BlockChildType {
    Text(TextCell),
    Delimited(DelimitedCell),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataCell {
    #[serde(skip)]
    pub id: u32,
    #[serde(flatten)]
    pub cell_type: CellType,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelimitedCell {
    pub delimeter: String,
    pub terminal: String,
}

pub trait Cell<T> {
    fn init_cell(id: u32, s: T) -> DataCell;
    fn push_cell(parent: &mut DataCell, id: u32, s: T);
    fn add_cell(add_to: &mut DataCell, parent_id: u32, id: u32, text: T);
}

impl ElementCell {
    fn push_attribute(&mut self, line: &str) {
        if let Some(prop_line) = line.split_once(" ") {
            self.props.push(Prop {
                key: prop_line.0.to_string(),
                value: prop_line.1.to_string(),
            })
        }
    }

    pub fn add_attribute(tree: &mut DataCell, cell_id: u32, prop_line: &str) {
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
}

impl Cell<&str> for ElementCell {
    fn init_cell(id: u32, tag_name: &str) -> DataCell {
        DataCell {
            id,
            cell_type: CellType::Element(ElementCell {
                name: tag_name.to_string(),
                ..Default::default()
            }),
        }
    }

    fn push_cell(parent: &mut DataCell, id: u32, tag_name: &str) {
        match parent.cell_type {
            CellType::Element(ref mut el) => el.children.push(Self::init_cell(id, tag_name)),
            CellType::Root(ref mut el) => el.children.push(Self::init_cell(id, tag_name)),
            _ => (),
        }
    }

    fn add_cell(add_to: &mut DataCell, parent_id: u32, id: u32, tag_name: &str) {
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
}

impl Cell<&BlockCell> for BlockCell {
    fn init_cell(id: u32, block: &BlockCell) -> DataCell {
        DataCell {
            id,
            cell_type: CellType::Block(block.to_owned()),
        }
    }

    fn push_cell(parent: &mut DataCell, id: u32, block: &BlockCell) {
        match parent.cell_type {
            CellType::Element(ref mut el) => el.children.push(Self::init_cell(id, block)),
            _ => (),
        }
    }

    fn add_cell(add_to: &mut DataCell, parent_id: u32, id: u32, block: &BlockCell) {
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
    //fn add_cell(add_to: &mut BlockCell, parent_id: u32, id: u32, text: &str);
}

impl BlockChild<BlockChildType> for BlockChildType {
    fn push_cell(parent: &mut BlockCell, block: BlockChildType) {
        parent.children.push(block)
    }
}

pub fn as_root(cell: &DataCell) -> Option<&Root> {
    match &cell.cell_type {
        CellType::Root(root) => Some(root),
        _ => None,
    }
}

pub fn get_cell_by_id(cell: &mut DataCell, id: u32) -> Option<&mut DataCell> {
    if cell.id == id {
        return Some(cell);
    };

    match &mut cell.cell_type {
        CellType::Element(ref mut el) => el.children.iter_mut().find_map(|x| {
            if let Some(child) = get_cell_by_id(x, id) {
                Some(child)
            } else {
                None
            }
        }),

        CellType::Root(ref mut el) => {
            let res = el.children.iter_mut().find_map(|x| {
                if let Some(child) = get_cell_by_id(x, id) {
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
