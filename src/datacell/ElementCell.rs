use super::{
    CellTrait::Cell,
    Datacell::{CellType, DataCell},
};
use serde::{Deserialize, Serialize};

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
                CellType::Root(ref mut el) => el.push_attribute(prop_line),
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
