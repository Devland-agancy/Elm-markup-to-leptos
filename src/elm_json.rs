use std::str::Lines;
use std::vec;

use super::element_text::ElementText;
use super::elm_json_helpers::*;
use super::helpers::*;

#[derive(Debug)]
pub struct ElmJSON {
    track_line_delta: usize,
    result: DataCell,
    depth_level: usize,
    id: u32,
}

impl ElmJSON {
    pub fn new() -> ElmJSON {
        Self {
            track_line_delta: 0,
            result: DataCell {
                id: 0,
                cell_type: CellType::Root(Root { children: vec![] }),
            },
            depth_level: 0,
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
        let mut track_line_index = 0;
        let mut text_node = String::new();

        for (index, line) in lines.clone().enumerate() {
            if lines_to_skip > 0 {
                lines_to_skip = lines_to_skip - 1;
                continue;
            }

            let trimmed_line = line.trim_start();
            let indent = get_line_indent(line);
            let current_cell: Option<&mut DataCell>;

            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].trim();
                tag_stack_pop(&mut tag_stack, &indent);

                curr_el_id = if let Some(last) = tag_stack.last() {
                    Some(last.id)
                } else if is_nested {
                    is_nested = false;
                    curr_el_id
                } else {
                    Some(0)
                };

                //let res_cloned = &mut self.result.clone();
                //current_cell = get_cell_by_id(res_cloned, curr_el_id);

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
                /* output.push_str(">\n\"\" ") ; */

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

                let last = tag_stack.last().expect(&format!(
                    "There is no parent tag . line {}",
                    track_line_index
                ));
                if last.in_props {
                    // tag props
                    ElementCell::add_attribute(&mut self.result, last.id, trimmed_line);

                    /* output.push_str(&format!("{}\n", line)); */
                    continue;
                }

                // tag content
                let mut inner_lines_to_skip: u32 = 0;
                /*
                check_indent_size(indent as isize, track_line_index + j as isize);
                check_extra_spaces(
                    indent,
                    tag_stack.last().unwrap().indent,
                    track_line_index + j as isize,
                ); */

                let next_line = lines.clone().nth(index + 1);
                let next_line_empty = next_line.is_some() && next_line.unwrap().is_empty();
                let next_line_is_element =
                    next_line.is_some() && next_line.unwrap().trim_start().starts_with("|> ");
                // break if next line is empty

                if next_line_empty
                    || next_line_is_element
                    || indent < tag_stack.last().unwrap().indent
                {
                    text_node = format!(
                        "{}{}{}",
                        text_node,
                        if text_node == "" { "" } else { " " },
                        trimmed_line.trim_end()
                    );

                    let mut block = BlockCell::new();
                    let e = ElementText::new(&text_node);
                    let block_children = e.split_text();
                    block_children.iter().for_each(|child| {
                        BlockChildType::push_cell(&mut block, child.to_owned());
                    });

                    BlockCell::add_cell(&mut self.result, curr_el_id.unwrap(), self.id, &block);
                    self.id += 1;

                    /*  output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n")); */
                    text_node = "".to_string();
                    continue;
                }

                text_node = format!(
                    "{}{}{}",
                    text_node,
                    if text_node == "" { "" } else { " " },
                    trimmed_line.trim_end()
                );

                // println!("noes {:#?}", nodes);

                /* output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n")); */
            }
        }
        let res = serde_json::to_string(&self.result);
        res.unwrap_or("Something Wrong".to_string())
    }

    fn handle_inline_element(
        &mut self,
        lines: Lines<'_>,
        start_from: usize,
        initial_indent: usize,
        mut curr_el_id: Option<u32>,
    ) -> (String, u32) {
        let mut element = "".to_string();
        let mut end_line = start_from;
        let mut inner_indent = initial_indent + 4;
        let mut skips: u32 = 0;
        let mut prev_line = "";

        while inner_indent > initial_indent
            || lines.clone().nth(end_line).unwrap().is_empty()
            || lines
                .clone()
                .nth(end_line)
                .unwrap()
                .chars()
                .all(char::is_whitespace)
        {
            if lines.clone().nth(end_line + 1).is_none() {
                break;
            }

            let inner_line = lines.clone().nth(end_line + 1).unwrap();
            let inner_trimmed_line = inner_line.trim_start();
            inner_indent = get_line_indent(inner_line);

            if inner_indent > initial_indent
                || inner_line.is_empty()
                || inner_line.chars().all(char::is_whitespace)
            {
                end_line += 1;
                skips += 1;
                continue;
            }
            break;
        }
        // do not skip empty line at the end
        let prev_line = lines.clone().nth(end_line).unwrap();

        if prev_line.is_empty() || prev_line.chars().all(char::is_whitespace) {
            skips -= 1
        }

        let mut i = start_from;
        for i in start_from..=end_line {
            element += &(lines.clone().nth(i).unwrap().to_string() + "\n");
        }
        element += &("".to_string() + "\n");
        //let save_id = curr_el_id;
        element = self.export_json(&element, curr_el_id, true);
        //curr_el_id = save_id;
        (element, skips)
    }
}
