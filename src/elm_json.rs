use std::str::Lines;

use super::element_text::ElementText;
use super::helpers::*;
use super::transform::TagInfo;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use syn::token::Enum;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
enum CellType {
    #[default]
    Default,
    Text(TextCell),
    Element(ElementCell),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct DataCell {
    cell_type: CellType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct TextCell {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Prop {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct ElementCell {
    name: String,
    props: Vec<Prop>,
    children: Vec<DataCell>,
}

#[derive(Debug)]
pub struct ElmJSON {
    track_line_delta: usize,
    result: Vec<DataCell>,
    depth_level: usize,
}

impl ElmJSON {
    pub fn new() -> ElmJSON {
        Self {
            track_line_delta: 0,
            result: Vec::new(),
            depth_level: 0,
        }
    }

    pub fn import_json(json: &String) -> String {
        json.clone()
    }

    pub fn export_json(&mut self, elm: &String) -> String {
        let mut tag_stack: Vec<TagInfo> = Vec::new();
        let lines = elm.lines();
        let mut lines_to_skip: u32 = 0;
        let mut track_line_index = 0;

        for (index, line) in lines.clone().enumerate() {
            if lines_to_skip > 0 {
                lines_to_skip = lines_to_skip - 1;
                continue;
            }

            let trimmed_line = line.trim_start();
            let indent = get_line_indent(line);

            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].trim();
                let is_new = Self::is_new_element(&mut tag_stack, &indent);

                if is_new {
                    self.depth_level -= if self.depth_level == 0 { 0 } else { 1 };
                } else {
                    self.depth_level += 1;
                }
                let res = self.result.clone();
                let mut current_element = res.iter().rev().find(|x| match x.cell_type {
                    CellType::Element(_) => true,
                    _ => false,
                });

                let default = &DataCell {
                    ..Default::default()
                };

                /*  if let Some(current) = current_element {
                    for iter in 0..self.depth_level {
                        // current_element is the last cell of type element in current depth_level
                        current_element = match &current.cell_type {
                            CellType::Element(el) => {
                                (el.children).iter().rev().find(move |x| match x.cell_type {
                                    CellType::Element(_) => {
                                        if iter == self.depth_level {
                                            el.children.push(DataCell {
                                                cell_type: CellType::Element(ElementCell {
                                                    name: tag_name.to_string(),
                                                    ..Default::default()
                                                }),
                                            });
                                        }
                                        return true;
                                    }
                                    _ => false,
                                })
                            }
                            _ => Some(default),
                        }
                    }
                } else {
                    self.result.push(DataCell {
                        cell_type: CellType::Element(ElementCell {
                            name: tag_name.to_string(),
                            ..Default::default()
                        }),
                    })
                } */

                /*   println!(
                    "{:?}",
                    serde_json::to_string(&current_element).unwrap_or("err".to_string())
                ); */

                tag_stack.push(TagInfo {
                    name: tag_name.to_string(),
                    indent,
                    is_self_closing: false,
                    in_props: true,
                });
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
                Self::is_new_element(&mut tag_stack, &indent);

                let last = tag_stack.last().expect(&format!(
                    "There is no parent tag . line {}",
                    track_line_index
                ));
                if last.in_props {
                    // tag props
                    /* output.push_str(&format!("{}\n", line)); */
                    continue;
                }

                // tag content
                let mut nodes: Vec<Vec<(String, bool)>> = Vec::new(); // if there is an inline element in the text , the text should be seperated to sub text before the element ( bool is added to know if the node is an element ) and the subtext after the element . it shouldn't be handeled as 1 block ,
                nodes.push(Vec::new());
                let mut text_node = String::new();
                let mut inner_lines_to_skip: u32 = 0;

                for (j, text_line) in lines.clone().skip(index).enumerate() {
                    if inner_lines_to_skip > 0 {
                        inner_lines_to_skip = inner_lines_to_skip - 1;
                        continue;
                    }

                    let inner_trimmed_line = text_line.trim_start();
                    let indent = get_line_indent(text_line);
                    check_indent_size(indent as isize, track_line_index + j as isize);
                    check_extra_spaces(
                        indent,
                        tag_stack.last().unwrap().indent,
                        track_line_index + j as isize,
                    );
                    // break if next line is new ( not nested ) element
                    if let Some(next_line) = lines.clone().nth(index + j + 1) {
                        let next_line_trimmed = next_line.trim_start();
                        let next_line_indent = get_line_indent(next_line);

                        if next_line_indent <= tag_stack.last().unwrap().indent
                            && next_line_trimmed.starts_with("|>")
                        {
                            if text_node != "" {
                                nodes.last_mut().unwrap().push((text_node.clone(), false));
                            }
                            break;
                        }
                    }

                    if inner_trimmed_line.is_empty() {
                        if text_node != "" {
                            nodes.last_mut().unwrap().push((text_node.clone(), false));
                            text_node = "".to_string();
                        }
                        lines_to_skip += 1;
                        nodes.push(Vec::new());
                        continue;
                    }

                    if indent < tag_stack.last().unwrap().indent {
                        if text_node != "" {
                            nodes.last_mut().unwrap().push((text_node.clone(), false));
                        }
                        break;
                    }

                    if !inner_trimmed_line.starts_with("|>") {
                        text_node += &format!(" {}", inner_trimmed_line);
                        lines_to_skip += 1;
                        continue;
                    }
                    if text_node != "" {
                        nodes.last_mut().unwrap().push((text_node, false));
                        text_node = "".to_string();
                    }

                    // handle inline element that can't be handled by delimiters ( they need props )
                    let (element, skips) =
                        self.handle_inline_element(lines.clone(), j + index, indent);

                    inner_lines_to_skip += skips;
                    lines_to_skip += skips + 1;

                    nodes
                        .last_mut()
                        .unwrap()
                        .push((format!("\"#{}r#\"", element), true));
                }
                let mut processed_text = String::new();
                for (node_idx, sub_nodes) in nodes.iter().enumerate() {
                    if sub_nodes.len() == 0 {
                        continue;
                    }
                    // subnodes are seperated with empty line
                    let mut sub_node_text = "".to_string();

                    for (sub_node_idx, (text, is_element)) in sub_nodes.iter().enumerate() {
                        if *is_element {
                            sub_node_text += &text;
                            continue;
                        }

                        sub_node_text += &ElementText::new(&text).handle_delimeters();
                    }

                    processed_text += sub_node_text.as_str();
                }
                nodes = vec![];
                /* output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n")); */
            }
        }
        let res = serde_json::to_string(&self.result);
        res.unwrap_or("Something Wrong".to_string())
    }

    pub fn is_new_element(tag_stack: &mut Vec<TagInfo>, indent: &usize) -> bool {
        while let Some(last_tag_info) = tag_stack.last() {
            if *indent <= last_tag_info.indent {
                tag_stack.pop();
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    fn handle_inline_element(
        &mut self,
        lines: Lines<'_>,
        start_from: usize,
        initial_indent: usize,
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
        element = self.export_json(&element);
        (element, skips)
    }
}
