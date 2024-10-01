use crate::datacell::{
    BlockCell::BlockCell, BlockChildType::*, CellTrait::Cell, Datacell::*, ElementCell::*,
};

pub struct Desugarer {
    pub json: String,
    pub last_id: usize,
}

pub struct ParagraphIndentOptions {
    pub tags_before_non_indents: Vec<&'static str>,
    pub tags_with_non_indent_first_child: Vec<&'static str>,
}

pub enum AttachToEnum {
    BEFORE,
    AFTER,
    BOTH,
    NONE,
}

pub struct IgnoreOptions {
    pub element: &'static str,
    pub attach_to: AttachToEnum,
}

impl Desugarer {
    pub fn new(json: &str, last_id: usize) -> Desugarer {
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

    fn find_cell_and_mark_article<'a>(
        &self,
        root: &'a DataCell,
        tag_names: &Vec<&str>,
        cells: &mut Vec<(usize, &'a DataCell)>, //usize is the article id
        mut article: Option<usize>,
        article_types: &Vec<String>,
    ) {
        match &root.cell_type {
            CellType::Element(el) => {
                if tag_names.is_empty() || tag_names.contains(&el.name.as_str()) {
                    if let Some(article_id) = article {
                        cells.push((article_id, root))
                    }
                }
                if article_types.contains(&el.name.chars().filter(|c| !c.is_numeric()).collect()) {
                    article = Some(root.id)
                }

                el.children.iter().for_each(|child| {
                    self.find_cell_and_mark_article(
                        child,
                        tag_names,
                        cells,
                        article,
                        &article_types,
                    )
                })
            }
            CellType::Root(el) => el.children.iter().for_each(|child| {
                self.find_cell_and_mark_article(child, tag_names, cells, article, &article_types)
            }),
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
            if count > 0 {
                for i in 1..=count - 1 {
                    prop_line += &format!(",\"{}\"", i);
                }
                prop_line += "]";

                //exercises_cell

                ElementCell::add_attribute(&mut root, exercises_cell.id, prop_line.as_str());
            }
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn add_increamental_attr(
        &mut self,
        tags_attributes: Vec<(&str, &str)>,
        article_types: &Vec<String>,
    ) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut elements: Vec<(usize, &DataCell)> = Vec::new();
        let binding = root.clone();
        let tag_names: Vec<&str> = tags_attributes.iter().map(|x| x.0).collect();
        let attribute_names: Vec<&str> = tags_attributes.iter().map(|x| x.1).collect();
        self.find_cell_and_mark_article(&binding, &tag_names, &mut elements, None, article_types);

        let mut counters = vec![0; tag_names.len()];

        for (i, (article_id, solution_cell)) in elements.iter().enumerate() {
            //get element position in tag_names
            if let CellType::Element(el) = &solution_cell.cell_type {
                let (tag_index, _) = tag_names
                    .iter()
                    .enumerate()
                    .find(|x| x.1 == &el.name)
                    .unwrap();

                // reset counter on new article
                if i > 0 && elements[i - 1].0 != *article_id {
                    counters[tag_index] = 0;
                }

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
        article_types: &Vec<String>,
    ) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut elements: Vec<(usize, &DataCell)> = Vec::new();
        let binding = root.clone();
        self.find_cell_and_mark_article(
            &binding,
            &vec![tag_name],
            &mut elements,
            None,
            article_types,
        );

        for (_, (article_id, element)) in elements.clone().iter_mut().enumerate() {
            if let CellType::Element(el) = &element.cell_type {
                // counter prop to parent if it doesn't have it

                ElementCell::add_attribute(
                    &mut root,
                    *article_id,
                    &format!("counter {}{}_counter", title_label, article_id),
                );

                let handle = el.props.iter().find(|x| x.key == "handle");

                let command = format!(
                    "{} {}::++{}{}_counter.",
                    title_label,
                    if handle.is_some() {
                        handle.unwrap().value.to_owned() + "<<"
                    } else {
                        "".to_string()
                    },
                    title_label,
                    article_id
                );

                let new_block_child = BlockChildType::Delimited(DelimitedCell {
                    open_delimeter: "*".to_string(),
                    close_delimeter: "*".to_string(),
                    terminal: command,
                    display_type: DelimitedDisplayType::INLINE,
                    wrapped_with: None,
                });

                // add space to element text ( first block )
                BlockChildType::insert_text_to_first_block_child_text(&mut root, element.id, " ");

                BlockChildType::add_block_at_first(
                    &mut root,
                    element.id,
                    &new_block_child,
                    Some(&BlockCell {
                        has_counter_commands: true,
                        ..Default::default()
                    }),
                );
            }
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
        // elements are what we want to wrap their children with wrap_with

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
                let mut no_wrap = false;

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
                            AttachToEnum::NONE => {
                                // Do nothing for these elements . just move to last so ordering doesn't change .
                                //no_wrap = true;
                                add_wrapper = false;
                                include_in_prev_wrapper = false;
                                ElementCell::move_cell(
                                    &mut root,
                                    (element.id, child.id),
                                    element.id,
                                );
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
                    // if no_wrap {

                    // }
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

    pub fn add_indent(&mut self) -> Desugarer {
        // A paragraph has an indent by default except:

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut _elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();

        self.find_cell(&binding, &vec!["Paragraph"], &mut _elements);

        let mut elements_to_indent: Vec<&DataCell> = Vec::new();

        for element in _elements.iter() {
            let parent: Option<&mut DataCell> =
                DataCell::get_cell_by_id(&mut root, element.parent_id);

            // an indent appears if this paragraph is preceded by a paragraph that has a non-block delimiter last child
            if Self::paragraph_first_child_is_text(&element)
                && Self::prev_is_non_block_delimiter(element.id, &parent)
            {
                elements_to_indent.push(element);
            }
        }

        for element in elements_to_indent.iter() {
            self.last_id += 1;
            ElementCell::add_cell(&mut root, element.id, self.last_id, "Indent");
            ElementCell::move_children(&mut root, element.id, self.last_id);
        }

        Desugarer {
            json: serde_json::to_string_pretty(&root).unwrap(),
            last_id: self.last_id,
        }
    }

    pub fn paragraph_first_child_is_text(element: &DataCell) -> bool {
        if let CellType::Element(el) = &element.cell_type {
            return el.children.first().is_some_and(|c| {
                if let CellType::Block(block) = &c.cell_type {
                    return block.children.first().is_some_and(|block_child| {
                        if let BlockChildType::Text(_) = &block_child {
                            return true;
                        }
                        false
                    });
                }
                false
            });
        }
        false
    }

    pub fn is_first_child(
        element_id: usize,
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

    fn prev_is_non_block_delimiter(element_id: usize, parent: &Option<&mut DataCell>) -> bool {
        if let Some(parent) = parent {
            if let CellType::Element(parent_el) = &parent.cell_type {
                let mut prev_el: Option<&DataCell> = None;
                for child in &parent_el.children {
                    if child.id == element_id {
                        break;
                    }
                    prev_el = Some(&child)
                }
                return prev_el.is_some_and(|p| {
                    if let CellType::Element(el) = &p.cell_type {
                        if el.name != "Paragraph" {
                            return false;
                        }
                        if let Some(block) = el.children.last() {
                            if let CellType::Block(block) = &block.cell_type {
                                if let Some(block) = block.children.last() {
                                    if let BlockChildType::Delimited(b) = &block {
                                        return b.display_type != DelimitedDisplayType::BLOCK
                                            && b.open_delimeter != "*";
                                    }
                                    if let BlockChildType::Text(_) = &block {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    false
                });
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
