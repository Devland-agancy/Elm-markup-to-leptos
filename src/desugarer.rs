use crate::{
    element_text,
    emitter::Emitter,
    parser_helpers::{
        BlockChild, BlockChildType, Cell, CellType, DataCell, DelimitedCell, DelimitedDisplayType,
        ElementCell,
    },
};

use super::helpers::*;

pub struct Desugarer {
    pub json: String,
    pub last_id: u32,
}

pub struct ParagraphIndentOptions {
    pub tags_before_non_indents: Vec<&'static str>,
    pub tags_with_non_indent_first_child: Vec<&'static str>,
}

pub enum AttachToEnum {
    BEFORE,
    AFTER,
    BOTH,
}
pub struct IgnoreOptions {
    pub element: &'static str,
    pub attach_to: AttachToEnum,
}

impl Desugarer {
    pub fn new(json: &str, last_id: u32) -> Desugarer {
        Desugarer {
            json: json.to_string(),
            last_id,
        }
    }

    fn count_element(&mut self, root: &DataCell, tag_name: &str, count: &mut usize) {
        match &root.cell_type {
            CellType::Element(el) => {
                if el.name == tag_name {
                    *count += 1;
                } else {
                    el.children
                        .iter()
                        .for_each(|child| self.count_element(child, tag_name, count))
                }
            }
            CellType::Root(el) => el
                .children
                .iter()
                .for_each(|child| self.count_element(child, tag_name, count)),

            _ => (),
        }
    }

    fn find_cell<'a>(
        &self,
        root: &'a DataCell,
        tag_names: &Vec<&str>,
        cells: &mut Vec<&'a DataCell>,
    ) {
        match &root.cell_type {
            CellType::Element(el) => {
                if tag_names.is_empty() || tag_names.contains(&el.name.as_str()) {
                    cells.push(root)
                }
                el.children
                    .iter()
                    .for_each(|child| self.find_cell(child, tag_names, cells))
            }
            CellType::Root(el) => el
                .children
                .iter()
                .for_each(|child| self.find_cell(child, tag_names, cells)),
            _ => (),
        }
    }

    pub fn pre_process_exercises(&mut self) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut exercises: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &vec!["Exercises"], &mut exercises);

        for exercises_cell in exercises.iter() {
            let mut count: usize = 0;
            self.count_element(exercises_cell, "Exercise", &mut count);
            let mut prop_line = "labels vec![\"0\"".to_string();
            for i in 1..=count - 1 {
                prop_line += &format!(",\"{}\"", i);
            }
            prop_line += "]";

            ElementCell::add_attribute(&mut root, exercises_cell.id, prop_line.as_str());
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn add_increamental_attr(&mut self, tags_attributes: Vec<(&str, &str)>) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        let tag_names: Vec<&str> = tags_attributes.iter().map(|x| x.0).collect();
        let attribute_names: Vec<&str> = tags_attributes.iter().map(|x| x.1).collect();
        self.find_cell(&binding, &tag_names, &mut elements);

        let mut counters = vec![0; tag_names.len()];

        for solution_cell in elements.iter() {
            //get element position in tag_names
            if let CellType::Element(el) = &solution_cell.cell_type {
                let (tag_index, _) = tag_names
                    .iter()
                    .enumerate()
                    .find(|x| x.1 == &el.name)
                    .unwrap();

                let prop_line = format!("{} {}", attribute_names[tag_index], counters[tag_index]);
                counters[tag_index] += 1;

                ElementCell::add_attribute(&mut root, solution_cell.id, prop_line.as_str());
            }
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn auto_increamental_title(
        &mut self,
        tag_name: &str,
        title_label: &str,
        // wrapper: Option<&str>,
        // wrapper_break_on: Option<&str>,
    ) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &vec![tag_name], &mut elements);

        for (i, element) in elements.clone().iter().enumerate() {
            let new_block_child = BlockChildType::Delimited(DelimitedCell {
                open_delimeter: "*".to_string(),
                close_delimeter: "*".to_string(),
                terminal: title_label.to_string() + " " + (i + 1).to_string().as_str() + ". ",
                display_type: DelimitedDisplayType::INLINE,
                wrapped_with: None,
            });

            BlockChildType::add_block_at_first(&mut root, element.id, &new_block_child);

            //BlockCell::add_cell_at_first(&mut root, element.id, self.last_id, &new_block);
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn wrap_children(
        &mut self,
        elements: Vec<&str>,
        wrap_with: &str,
        ignore_elements: &Option<Vec<IgnoreOptions>>,
    ) -> Desugarer {
        // elements are what we want to wrap it's children with wrap_with

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut _elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &elements, &mut _elements);

        for (_, element) in _elements.iter().enumerate() {
            //let prop_line = format!("solution_number {}", i);
            if let CellType::Element(el) = &element.cell_type {
                let mut include_prev_child = false;
                let mut include_in_prev_wrapper = false;
                let mut add_wrapper = true;

                el.children.iter().enumerate().for_each(|(idx, child)| {
                    let mut element_ignored = None;
                    if let Some(ignore_options) = ignore_elements {
                        ignore_options.iter().any(|option| {
                            if let CellType::Element(child_el) = &child.cell_type {
                                if option.element == child_el.name {
                                    element_ignored = Some(option);
                                    add_wrapper = false;
                                    return true;
                                }
                            }
                            add_wrapper = true;
                            false
                        });
                    }

                    if let Some(el_ignored) = element_ignored {
                        match el_ignored.attach_to {
                            AttachToEnum::BEFORE => {
                                // move to previous added wrapper if there is one
                                if idx == 0 {
                                    add_wrapper = true;
                                } else {
                                    ElementCell::move_cell(
                                        &mut root,
                                        (element.id, child.id),
                                        self.last_id,
                                    );
                                }
                            }
                            AttachToEnum::AFTER => {
                                // move to next wrapper
                                include_prev_child = true
                            }
                            AttachToEnum::BOTH => {
                                // this and next block should be in previous added wrapper
                                if idx == 0 {
                                    self.last_id += 1;
                                    ElementCell::add_cell(
                                        &mut root,
                                        element.id,
                                        self.last_id,
                                        wrap_with,
                                    );
                                }
                                ElementCell::move_cell(
                                    // current child to previous wrapper
                                    &mut root,
                                    (element.id, child.id),
                                    self.last_id,
                                );
                                include_in_prev_wrapper = true // next child to previous wrapper
                            }
                        }
                    } else if include_in_prev_wrapper {
                        ElementCell::move_cell(&mut root, (element.id, child.id), self.last_id);
                        include_in_prev_wrapper = false;

                        add_wrapper = false
                    }

                    if add_wrapper {
                        self.last_id += 1;
                        ElementCell::add_cell(&mut root, element.id, self.last_id, wrap_with);

                        if include_prev_child {
                            ElementCell::move_cell(
                                &mut root,
                                (element.id, el.children[idx - 1].id),
                                self.last_id,
                            );
                            include_prev_child = false
                        }
                        ElementCell::move_cell(&mut root, (element.id, child.id), self.last_id);
                    }
                });
            }
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn wrap_block_delimited(&self, wrap_with: &str) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        self.wrap_recursive(&mut root, wrap_with);

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn wrap_recursive<'a>(&self, root: &'a mut DataCell, wrap_with: &str) -> Desugarer {
        match &mut root.cell_type {
            CellType::Block(block) => {
                block
                    .children
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, child)| {
                        if let BlockChildType::Delimited(dl) = child {
                            if i > 0 && dl.display_type == DelimitedDisplayType::BLOCK {
                                dl.wrapped_with = Some(wrap_with.to_string());
                            }
                        }
                    });
            }
            CellType::Element(el) => el.children.iter_mut().for_each(|child| {
                self.wrap_recursive(child, wrap_with);
            }),
            CellType::Root(el) => el.children.iter_mut().for_each(|child| {
                self.wrap_recursive(child, wrap_with);
            }),
            _ => (),
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn add_indent(&mut self, options: &ParagraphIndentOptions) -> Desugarer {
        // A paragraph has an indent by default except:

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut _elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();

        self.find_cell(&binding, &vec!["Paragraph"], &mut _elements);

        for (element) in _elements.iter() {
            let parent: Option<&mut DataCell> =
                DataCell::get_cell_by_id(&mut root, element.parent_id);

            // only paragraph elements that have block cell
            if Self::paragraph_of_blocks(element) {
                continue;
            }
            // the first paragraph of every section, exercise, or example does not have an indent
            if Self::is_first_child(element.id, &parent, options) {
                continue;
            }
            // $$, __,  |_ paragraphs
            if Self::is_delimited(element, true) {
                continue;
            }
            // a paragraph that follows a paragraph ending with the $$, __,  |_ delimeters does not have an indent
            if Self::prev_is_delimited(element.id, &parent) {
                continue;
            }
            // a paragraph that follows tags defined in
            if Self::tags_before_non_indents(element.id, &parent, &options) {
                continue;
            }

            self.last_id += 1;
            ElementCell::add_cell(&mut root, element.id, self.last_id, "Indent");
            ElementCell::move_children(&mut root, element.id, self.last_id);
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn paragraph_of_blocks(element: &DataCell) -> bool {
        if let CellType::Element(el) = &element.cell_type {
            if el.children.first().is_some_and(|c| {
                if let CellType::Block(_) = &c.cell_type {
                    false
                } else {
                    true
                }
            }) {
                return true;
            }
        }
        false
    }

    pub fn is_first_child(
        element_id: u32,
        parent: &Option<&mut DataCell>,
        options: &ParagraphIndentOptions,
    ) -> bool {
        if let Some(parent) = parent {
            if let CellType::Element(parent) = &parent.cell_type {
                if options
                    .tags_with_non_indent_first_child
                    .contains(&parent.name.as_str())
                    && parent.children.first().is_some_and(|c| c.id == element_id)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_delimited(element: &DataCell, first_should_be_block: bool) -> bool {
        if let CellType::Element(element) = &element.cell_type {
            if let Some(block) = element.children.first() {
                if let CellType::Block(block) = &block.cell_type {
                    let first_child_is_block = block.children.first().is_some_and(|first| {
                        if let BlockChildType::Delimited(b) = first {
                            return b.display_type == DelimitedDisplayType::BLOCK;
                        } else {
                            false
                        }
                    });

                    if block.children.len() > 1 && !first_child_is_block && !first_should_be_block {
                        return false;
                    }

                    if block.children.len() > 1 && first_child_is_block && first_should_be_block {
                        return true;
                    }

                    if let Some(block) = block.children.first() {
                        if let BlockChildType::Delimited(b) = &block {
                            return b.display_type == DelimitedDisplayType::BLOCK
                                || b.open_delimeter == "*";
                        }
                    }
                }
            }
        }
        false
    }

    pub fn prev_is_delimited(element_id: u32, parent: &Option<&mut DataCell>) -> bool {
        if let Some(parent) = parent {
            if let CellType::Element(parent) = &parent.cell_type {
                let mut prev_el: Option<&DataCell> = None;
                for child in &parent.children {
                    if child.id == element_id {
                        break;
                    }
                    prev_el = Some(&child)
                }
                if prev_el.is_some_and(|p| Self::is_delimited(p, false)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn tags_before_non_indents(
        element_id: u32,
        parent: &Option<&mut DataCell>,
        option: &ParagraphIndentOptions,
    ) -> bool {
        if let Some(parent) = parent {
            if let CellType::Element(parent) = &parent.cell_type {
                let mut prev_el: Option<&DataCell> = None;
                for child in &parent.children {
                    if child.id == element_id {
                        break;
                    }
                    prev_el = Some(&child)
                }
                if let Some(prev_el) = prev_el {
                    if let CellType::Element(prev_el) = &prev_el.cell_type {
                        if prev_el.children.first().is_some_and(|p| {
                            if let CellType::Element(el) = &p.cell_type {
                                return option.tags_before_non_indents.contains(&el.name.as_str());
                            } else {
                                false
                            }
                        }) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn add_attribute(&mut self, tag_names: Vec<&str>, attribute: (&str, &str)) -> Desugarer {
        // elements are what we want to wrap it's children with wrap_with

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut _elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &tag_names, &mut _elements);

        for (_, element) in _elements.iter().enumerate() {
            //let prop_line = format!("solution_number {}", i);
            if let CellType::Element(_) = &element.cell_type {
                ElementCell::add_attribute(
                    &mut root,
                    element.parent_id,
                    &format!("{} {}", attribute.0, attribute.1),
                )
            }
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn auto_convert_to_float(&mut self, attrs: Vec<&str>) -> Desugarer {
        fn convert_recusive(root: &mut DataCell, attrs: &Vec<&str>) {
            match &mut root.cell_type {
                CellType::Element(el) => {
                    el.props.iter_mut().for_each(|prop| {
                        if attrs.contains(&prop.key.as_str()) && !prop.value.contains('.') {
                            prop.value = format!("{}.0", prop.value);
                        }
                    });
                    el.children
                        .iter_mut()
                        .for_each(|child| convert_recusive(child, attrs))
                }
                CellType::Root(el) => el
                    .children
                    .iter_mut()
                    .for_each(|child| convert_recusive(child, attrs)),
                _ => (),
            }
        }

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        convert_recusive(&mut root, &attrs);
        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }
}
