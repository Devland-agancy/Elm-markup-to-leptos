use leptos::html::Data;

use crate::{
    emitter::Emitter,
    parser_helpers::{BlockCell, BlockChildType, Cell, CellType, DataCell, ElementCell, Prop},
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
                } else {
                    el.children
                        .iter()
                        .for_each(|child| self.find_cell(child, tag_names, cells))
                }
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
            for i in 1..=count {
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

    pub fn pre_process_solutions(&mut self) -> Desugarer {
        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut solutions: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &vec!["Solution"], &mut solutions);

        for (i, solution_cell) in solutions.iter().enumerate() {
            let prop_line = format!("solution_number {}", i);

            ElementCell::add_attribute(&mut root, solution_cell.id, prop_line.as_str());
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
        options: Option<&ParagraphIndentOptions>,
    ) -> Desugarer {
        // elements are what we want to wrap it's children with wrap_with

        let mut root: DataCell = serde_json::from_str(&self.json).unwrap();
        let mut _elements: Vec<&DataCell> = Vec::new();
        let binding = root.clone();
        self.find_cell(&binding, &elements, &mut _elements);

        for (i, element) in _elements.iter().enumerate() {
            //let prop_line = format!("solution_number {}", i);
            if let CellType::Element(el) = &element.cell_type {
                el.children.iter().for_each(|child| {
                    self.last_id += 1;
                    // Custom handling for Paragraph - paragraphs that do not meet conditions are wrapped inside Indent
                    //if options.is_some() && self.add_indent(options.unwrap()) {
                    //    ElementCell::add_cell(&mut root, element.id, self.last_id, wrap_with);
                    //    ElementCell::add_cell(&mut root, self.last_id, self.last_id + 1, "Indent");
                    //    self.last_id += 1;
                    //} else {
                    //    ElementCell::add_cell(&mut root, element.id, self.last_id, wrap_with);
                    //}
                    ElementCell::add_cell(&mut root, element.id, self.last_id, wrap_with);
                    ElementCell::move_cell(&mut root, (element.id, child.id), self.last_id)
                });
            }
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
        /*  let mut tags_to_find = vec!["Paragraph"];
        tags_to_find.extend(options.tags_with_non_indent_first_child); */

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
            if Self::is_delimited(element) {
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

    pub fn is_delimited(element: &DataCell) -> bool {
        if let CellType::Element(element) = &element.cell_type {
            if let Some(block) = element.children.first() {
                if let CellType::Block(block) = &block.cell_type {
                    if let Some(block) = block.children.first() {
                        if let BlockChildType::Delimited(b) = &block {
                            return b.delimeter == "$$"
                                || b.delimeter == "__"
                                || b.delimeter == "_|";
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
                if prev_el.is_some_and(|p| Self::is_delimited(p)) {
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

    /*    pub fn pre_process_exercises(&mut self) -> Desugarer {
           let mut lines: Vec<String> = self.json.lines().map(|s| s.to_string()).collect();
           let binding = lines.clone();

           /* Wrap exercises inside Exercises component */
           /* Right now this works only if there are consuctive exercises */
           let mut exercises = binding
               .iter()
               .enumerate()
               .filter(|(_, line)| line.trim() == "|> Exercise");

           if let Some(exo) = exercises.nth(0) {
               // add prop line which is like labels=vec!["0", "1", "2", "3"]
               let mut props_string = "    labels=vec![\"0\"".to_string();
               for i in 1..exercises.clone().count() + 1 {
                   props_string += &format!(",\"{}\"", i);
               }
               props_string += "]";

               lines.insert(exo.0 - 1, props_string);
           }

           Desugarer {
               json: lines.join("\n"),
           }
       }

       pub fn pre_process_solutions(&mut self) -> Desugarer {
           let mut lines: Vec<String> = self.json.lines().map(|s| s.to_string()).collect();
           let binding = lines.clone();

           let mut solutions = binding
               .iter()
               .enumerate()
               .filter(|(_, line)| line.trim() == "|> Solution");

           for i in 0..solutions.clone().count() {
               let solution = solutions.next().unwrap();
               let solution_tag_line = solution.0 + i;

               let indent = solution.1.len() - solution.1.trim_start().len();
               let mut indent_string = "    ".to_string();
               for _ in 0..indent {
                   indent_string += " ";
               }
               let props_string = format!("{}solution_number={}", indent_string, i);
               lines.insert(solution_tag_line + 1, props_string);
           }
           Desugarer {
               json: lines.join("\n"),
           }
       }

       pub fn remove_empty_line_above(
           &mut self,
           tags: Vec<&str>,
           ignore_prop: Option<(&str, &str)>, // (key, value)
           parser: &mut Emitter,
       ) -> Desugarer {
           // Removes empty lines above tags

           let mut lines: Vec<String> = self.json.lines().map(|s| s.to_string()).collect();
           let binding = lines.clone();
           let mut lines_removed = 0;

           binding
               .iter()
               .enumerate()
               .filter(|(i, line)| {
                   let tag_found = line.trim().starts_with("|> ") && tags.contains(&&line.trim()[3..]);
                   let mut should_ignore = false;

                   if tag_found && ignore_prop.is_some() {
                       // search props
                       let indent = get_line_indent(line);
                       let mut j = *i + 1;
                       let next_line = binding.iter().nth(j);
                       if let Some(mut next_line) = next_line {
                           let mut next_line_indent = get_line_indent(next_line);

                           while next_line_indent > indent && !should_ignore {
                               let prop_key = next_line.trim().split_once(" ");
                               if let Some(prop_key) = prop_key {
                                   should_ignore = prop_key.0 == ignore_prop.unwrap().0
                                       && prop_key.1 == ignore_prop.unwrap().1
                               }
                               j += 1;
                               if binding.iter().nth(j).is_none() {
                                   break;
                               }
                               next_line = binding.iter().nth(j).unwrap();
                               next_line_indent = get_line_indent(next_line);
                           }
                       }
                   }
                   tag_found && !should_ignore
               })
               .for_each(|tag| {
                   lines.remove(tag.0 - 1 - lines_removed);
                   lines_removed += 1;
                   parser.track_line_delta -= 1
               });

           Desugarer {
               json: lines.join("\n"),
           }
       }

       pub fn auto_increamental_title(
           &mut self,

           tag_name: &str,
           title_label: &str,
           wrapper: Option<&str>,
           wrapper_break_on: Option<&str>,
       ) -> Desugarer {
           let mut lines: Vec<String> = self.json.lines().map(|s| s.to_string()).collect();
           let binding = lines.clone();
           let mut jump = 2;

           let _ = binding
               .iter()
               .enumerate()
               .filter(|(_, line)| line.trim() == format!("|> {}", tag_name))
               .enumerate()
               .for_each(|(idx, ex)| {
                   // Suppose there are not props for Example , we add title at 3rd line
                   let indent = ex.1.len() - ex.1.trim_start().len();
                   let mut indent_string = "    ".to_string();
                   for _ in 0..indent {
                       indent_string += " ";
                   }
                   if let Some(wrapper) = wrapper {
                       lines.insert(
                           ex.0 + jump + idx,
                           format!("{}|> {}", indent_string, wrapper),
                       );
                       lines.insert(ex.0 + jump + 1 + idx, "".to_string());
                       indent_string += "    ";
                       jump += 2;

                       let mut i = ex.0 + jump + idx;

                       while i < lines.len()
                           && lines[i].trim() != format!("|> {}", wrapper_break_on.unwrap())
                           && (lines[i].is_empty()
                               || lines[i].chars().all(char::is_whitespace)
                               || lines[i].len() - lines[i].trim_start().len() > indent)
                       {
                           i += 1;
                       }

                       for i in ex.0 + jump + idx..i {
                           if lines[i].is_empty() || lines[i].chars().all(char::is_whitespace) {
                               continue;
                           };
                           lines[i] = format!("    {}", lines[i]);
                       }
                   }
                   lines.insert(
                       ex.0 + jump + idx,
                       format!("{}*{} {}.*", indent_string, title_label, idx + 1),
                   );
               });

           Desugarer {
               json: lines.join("\n"),
           }
       }
    */
}
