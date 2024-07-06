use std::str::Lines;
use std::vec;

use super::element_text::ElementText;
use super::helpers::*;
use super::parser_helpers::*;

#[derive(Debug)]
pub struct Parser {
    track_line_delta: usize,
    result: DataCell,
    pub id: u32,
}

impl Parser {
    pub fn new() -> Parser {
        Self {
            track_line_delta: 0,
            result: DataCell {
                parent_id: 0,
                id: 0,
                cell_type: CellType::Root(Root { children: vec![] }),
            },
            id: 1,
        }
    }

    pub fn import_json(json: &String) -> String {
        json.clone()
    }

    pub fn export_json(
        &mut self,
        elm: &String,
        mut curr_el_id: Option<u32>,
        mut is_nested: bool,
    ) -> String {
        let mut tag_stack: Vec<TagInfo> = Vec::new();
        let lines = elm.lines();
        let mut lines_to_skip: u32 = 0;
        let mut text_node = String::new();

        for (index, line) in lines.clone().enumerate() {
            if lines_to_skip > 0 {
                lines_to_skip = lines_to_skip - 1;
                continue;
            }

            let trimmed_line = line.trim_start();
            let indent = get_line_indent(line);

            check_indent_size(indent as isize, line);
            if let Some(last) = tag_stack.last() {
                check_extra_spaces(indent, last.indent, line);
            }

            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].trim();
                //println!("line {}", line);
                //println!("line {:#?}", tag_stack.last());
                tag_stack_pop(&mut tag_stack, &indent);

                curr_el_id = if let Some(last) = tag_stack.last() {
                    Some(last.id)
                } else if is_nested {
                    is_nested = false;
                    curr_el_id
                } else {
                    Some(0)
                };

                ElementCell::add_cell(&mut self.result, curr_el_id.unwrap(), self.id, tag_name);

                tag_stack.push(TagInfo {
                    id: self.id,
                    name: tag_name.to_string(),
                    indent,
                    is_self_closing: false,
                    in_props: true,
                });
                self.id += 1;

                continue;
            }
            if trimmed_line.is_empty() // end of props
            && tag_stack
                .last()
                .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
            {
                if let Some(last) = tag_stack.last_mut() {
                    last.in_props = false;
                }
                continue;
            }

            if !trimmed_line.is_empty() {
                tag_stack_pop(&mut tag_stack, &indent);

                curr_el_id = if let Some(last) = tag_stack.last() {
                    Some(last.id)
                } else if is_nested {
                    is_nested = false;
                    curr_el_id
                } else {
                    Some(0)
                };

                let last = tag_stack
                    .last()
                    .expect(&format!("There is no parent tag . at line \n {:?}", line));
                if last.in_props {
                    // tag props
                    ElementCell::add_attribute(&mut self.result, last.id, trimmed_line);
                    continue;
                }

                let next_line = lines.clone().nth(index + 1);
                let next_line_empty = next_line.is_none()
                    || (next_line.is_some() && next_line.unwrap().trim().is_empty());
                let next_line_is_element =
                    next_line.is_some() && next_line.unwrap().trim_start().starts_with("|> ");
                let next_line_indent = if let Some(next_line) = next_line {
                    Some(get_line_indent(next_line))
                } else {
                    None
                };

                let end_of_attached_element =
                    next_line_indent.is_some_and(|ne| ne > 0 && ne < indent);

                // break if next line is empty
                if next_line_empty
                    || next_line_is_element
                    || indent < tag_stack.last().unwrap().indent
                    || end_of_attached_element
                {
                    text_node = format!(
                        "{}{}{}",
                        text_node,
                        if text_node == "" { "" } else { " " },
                        trimmed_line.trim_end(),
                    );

                    let mut block = BlockCell::new();
                    let e = ElementText::new(&text_node);
                    let block_children = e.split_text();

                    block_children.iter().for_each(|child| {
                        if let BlockChildType::Text(text) = &child {
                            if text.content != "" {
                                BlockChildType::push_cell(&mut block, child.to_owned());
                            }
                        } else {
                            BlockChildType::push_cell(&mut block, child.to_owned());
                        }
                    });

                    BlockCell::add_cell(&mut self.result, curr_el_id.unwrap(), self.id, &block);
                    self.id += 1;

                    if end_of_attached_element && tag_stack.len() > 1 {
                        let parent_id =
                            if let Some(before_last) = tag_stack.get(tag_stack.len() - 2) {
                                Some(before_last.id)
                            } else {
                                None
                            };
                        ElementCell::add_cell(
                            &mut self.result,
                            parent_id.unwrap(),
                            self.id,
                            "Space",
                        );
                        self.id += 1;
                    }

                    text_node = "".to_string();
                    continue;
                }

                text_node = format!(
                    "{}{}{}",
                    text_node,
                    if text_node == "" { "" } else { " " },
                    trimmed_line.trim_end()
                );
            }
        }
        let res = serde_json::to_string_pretty(&self.result);
        res.unwrap_or("Something Wrong".to_string())
    }
}
