use std::cell::Cell;

use leptos::html::Data;

use crate::elm_json::*;

pub fn as_root(cell: &DataCell) -> Option<&Root> {
    match &cell.cell_type {
        CellType::Root(root) => Some(root),
        _ => None,
    }
}

pub fn init_element_cell(id: u32, tag_name: &str) -> DataCell {
    DataCell {
        id,
        cell_type: CellType::Element(ElementCell {
            name: tag_name.to_string(),
            ..Default::default()
        }),
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

pub fn push_new_cell(parent: &mut DataCell, id: u32, tag_name: &str) {
    match parent.cell_type {
        CellType::Element(ref mut el) => el.children.push(init_element_cell(id, tag_name)),
        CellType::Root(ref mut el) => el.children.push(init_element_cell(id, tag_name)),
        _ => (),
    }
}

pub fn push_attribute(cell: &mut DataCell, line: &str) {
    if let Some(prop_line) = line.split_once(" ") {
        match cell.cell_type {
            CellType::Element(ref mut el) => el.props.push(Prop {
                key: prop_line.0.to_string(),
                value: prop_line.1.to_string(),
            }),
            _ => (),
        }
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
