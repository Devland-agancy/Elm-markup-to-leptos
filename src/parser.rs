use std::str::Lines;
use std::vec;

use crate::counter::counter_commands::CounterCommand;

use super::counter::counter_instance::CounterInstance;
use super::counter::counter_types::CounterType;
use super::counter::counters::Counters;

use super::element_text::ElementText;
use super::helpers::*;
use super::parser_helpers::*;

#[derive(Debug)]
pub struct Parser<'a> {
    result: DataCell,
    pub id: u32,
    pub counters: &'a mut Counters,
}

impl<'a> Parser<'a> {
    pub fn new(counters: &'a mut Counters) -> Parser<'a> {
        Self {
            result: DataCell {
                parent_id: 0,
                id: 0,
                cell_type: CellType::Root(Root { children: vec![] }),
            },
            id: 1,
            counters,
        }
    }

    pub fn import_json(json: &String) -> String {
        json.clone()
    }

    fn handle_tag_line(
        &mut self,
        trimmed_line: &str,
        mut curr_el_id: Option<u32>,
        mut is_nested: &bool,
        tag_stack: &mut Vec<TagInfo>,
        indent: usize,
    ) {
        let tag_name = trimmed_line[3..].trim();
        //println!("line {}", line);
        //println!("line {:#?}", tag_stack.last());
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
        elm: &'a str,
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

            //remove counters that are out of scope
            for counter in self.counters.clone().counters_list.iter_mut() {
                if !line.is_empty()
                    && !line.chars().all(char::is_whitespace)
                    && counter.scope > indent / 4
                {
                    self.counters.remove_counter(counter);
                }
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
                    .expect(&format!("There is no parent tag . at line \n {:?}", line));
                if last.in_props {
                    // tag props
                    let mut prop_line_splits = trimmed_line.split(" ");
                    let counter_type = prop_line_splits.next().unwrap();
                    if CounterType::is_valid(counter_type) {
                        let counter_name =
                            prop_line_splits.next().expect("Counter must have a name");
                        let default_value = prop_line_splits.next();
                        // create new counter
                        self.counters.add_counter(CounterInstance::new(
                            counter_name,
                            counter_type,
                            indent / 4,
                            default_value,
                        ));
                    }

                    ElementCell::add_attribute(&mut self.result, last.id, trimmed_line);
                    continue;
                }

                // counter
                //handle_counters();
                // if let Some(parent_id) = curr_el_id {
                //     let cc = CounterCommand::new();
                //     cc.generate_counter_elements(
                //         line,
                //         &mut self.counters,
                //         &mut self.result,
                //         &mut self.id,
                //         parent_id,
                //     );

                // };

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

                    // let cc = CounterCommand::new();
                    // let splits_by_command = cc.split_line(&text_node);

                    // splits_by_command.iter().for_each(|part| {
                    //     if part.is_command {
                    //         if let Some(parent_id) = curr_el_id {
                    //             cc.generate_counter_elements(
                    //                 &part.content,
                    //                 &mut self.counters,
                    //                 &mut self.result,
                    //                 &mut self.id,
                    //                 parent_id,
                    //             );
                    //         };
                    //     } else {

                    //     }
                    // });

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
                    trimmed_line.trim_end()
                );
            }
        }
        let res = serde_json::to_string_pretty(&self.result);
        res.unwrap_or("Something Wrong".to_string())
    }

    fn get_parent_id(&self, tag_stack: &Vec<TagInfo>) -> Option<u32> {
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
