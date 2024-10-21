use super::{
    BlockCell::BlockCell,
    ElementCell::{ElementCell, Prop},
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Root {
    #[serde(rename = "attributes")]
    pub children: Vec<DataCell>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataCell {
    pub id: usize,
    pub parent_id: usize,
    #[serde(flatten)]
    pub cell_type: CellType,
}

pub trait FlatElement {
    fn id(&self) -> usize;
    fn parent_id(&self) -> usize;
    fn children(&self) -> Option<&Vec<DataCell>>;
    fn props(&self) -> Option<&Vec<Prop>>;
    fn name(&self) -> Option<&String>;

    fn update_children(&mut self, new: Vec<DataCell>) -> Result<(), &str>;
}

impl FlatElement for DataCell {
    fn id(&self) -> usize {
        self.id
    }

    fn parent_id(&self) -> usize {
        self.parent_id
    }

    fn children(&self) -> Option<&Vec<DataCell>> {
        match &self.cell_type {
            CellType::Element(el) => Some(&el.children),
            CellType::Root(el) => Some(&el.children),
            _ => None
        }
    }

    fn update_children(&mut self, new: Vec<DataCell>) -> Result<(), &str> {
         match &mut self.cell_type {
            CellType::Element(el) => {
                el.children = new;
                Ok(())
            },
            CellType::Root(el) => {
                el.children = new;
                Ok(())
            },
            _ => Err("Cannot update children of this type of cell")
        }
    }

    fn name(&self) -> Option<&String> {
        if let CellType::Element(el) = &self.cell_type{
            return Some(&el.name)
        }
        None
    }

    fn props(&self) -> Option<&Vec<Prop>> {
         if let CellType::Element(el) = &self.cell_type{
            return Some(&el.props)
        }
        None
    }
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
