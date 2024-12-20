use crate::counter::counter_commands::CounterCommand;

use super::datacell::{
    BlockCell::BlockCell, BlockChildType::*, CellTrait::Cell, Datacell::*, ElementCell::*,
};
use super::element_text::ElementText;
use super::helpers::*;
use super::parser_helpers::{tag_stack_pop, TagInfo};

#[derive(Debug)]
pub struct Parser {
    result: DataCell,
    pub id: usize,
    //reset_line_tracking_on: String // used for tracking error line on merged input files . when input files are merged we reset the line on every element related to file
}

#[derive(Debug)]
pub enum ParserError {
    ExtraSpacesError(usize),
    None4xSpacesError(usize),
}

impl ParserError {
    pub fn to_string(&self) -> String {
        match self {
            ParserError::ExtraSpacesError(line) => format!("Extra spaces found on line {}", line),
            ParserError::None4xSpacesError(line) => {
                format!("None 4x spaces found on line {}", line)
            }
        }
    }

    pub fn to_string_without_line(&self) -> String {
        match self {
            ParserError::ExtraSpacesError(_) => format!("Extra spaces found"),
            ParserError::None4xSpacesError(_) => {
                format!("None 4x spaces found")
            }
        }
    }
}

impl Parser {
    pub fn new() -> Parser {
        Self {
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

    fn handle_tag_line(
        &mut self,
        trimmed_line: &str,
        mut curr_el_id: Option<usize>,
        mut is_nested: &bool,
        tag_stack: &mut Vec<TagInfo>,
        indent: usize,
    ) {
        let tag_name = trimmed_line[3..].trim();
        tag_stack_pop(tag_stack, &indent);

        curr_el_id = if let Some(last) = tag_stack.last() {
            Some(last.id)
        } else if *is_nested {
            is_nested = &false;
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
    }

    fn props_end(&mut self, trimmed_line: &str, tag_stack: &Vec<TagInfo>) -> bool {
        trimmed_line.is_empty() // end of props
        && tag_stack
            .last()
            .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
    }

    pub fn export_json(
        &mut self,
        elm: &str,
        mut curr_el_id: Option<usize>,
        mut is_nested: bool,
    ) -> Result<String, ParserError> {
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

            if trimmed_line.starts_with("!!") {
                continue;
            }
            check_indent_size(indent as isize, index)?;
            if let Some(last) = tag_stack.last() {
                check_extra_spaces(indent, last.indent, index)?;
            }

            if trimmed_line.starts_with("|> ") {
                self.handle_tag_line(trimmed_line, curr_el_id, &is_nested, &mut tag_stack, indent);
                continue;
            }
            if self.props_end(trimmed_line, &tag_stack) {
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
                    .expect(&format!("There is no parent tag . at line \n {:?}", index));
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
                        // if there's one space we keep it
                        if trimmed_line.trim_end().len() + 1 == trimmed_line.len() {
                            trimmed_line
                        } else {
                            trimmed_line.trim_end()
                        },
                    );

                    let mut block = BlockCell::new();

                    let e = ElementText::new(&text_node);
                    let block_children = e.split_text();

                    // mark block with counter syntax to avoid seaching all blocks later
                    // if CounterCommand::has_counter_syntax(text_node.as_str()) {
                    //     block.has_counter_commands = true;
                    // }
                    block.has_counter_commands =
                        CounterCommand::has_counter_syntax(text_node.as_str());
                    block.has_handle_insert =
                        CounterCommand::has_handle_insert_syntax(text_node.as_str());

                    block_children.iter().for_each(|child| {
                        if let BlockChildType::Text(text) = &child {
                            if text.content != "" {
                                BlockChildType::push_cell(&mut block, child.to_owned());
                            }
                        } else {
                            BlockChildType::push_cell(&mut block, child.to_owned());
                        }
                    });

                    text_node = "".to_string();

                    BlockCell::add_cell(&mut self.result, curr_el_id.unwrap(), self.id, &block);
                    self.id += 1;

                    if end_of_attached_element && tag_stack.len() > 1 {
                        let parent_id = self.get_parent_id(&tag_stack);
                        ElementCell::add_cell(
                            &mut self.result,
                            parent_id.unwrap(),
                            self.id,
                            "Space",
                        );
                        self.id += 1;
                    }

                    continue;
                }

                text_node = format!(
                    "{}{}{}",
                    text_node,
                    if text_node == "" { "" } else { " " },
                    if trimmed_line.trim_end().len() + 1 == trimmed_line.len() {
                        trimmed_line
                    } else {
                        trimmed_line.trim_end()
                    },
                );
                if !text_node.is_empty() {
                    text_node.push_str("\n")
                }
            }
        }
        let res = serde_json::to_string_pretty(&self.result);
        Ok(res.unwrap_or("Something Wrong".to_string()))
    }

    fn get_parent_id(&self, tag_stack: &Vec<TagInfo>) -> Option<usize> {
        let before_last_index = tag_stack.len().checked_sub(2);
        if before_last_index.is_none() {
            return None;
        }
        let before_last_index = before_last_index.unwrap();
        if let Some(before_last) = tag_stack.get(before_last_index) {
            Some(before_last.id)
        } else {
            None
        }
    }
}
