use std::str::Lines;
use std::vec;

use super::element_text::ElementText;
use super::elm_json_helpers::*;
use super::helpers::*;

use select::document::Document;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde_json::*;
use syn::token::Enum;

pub struct TagInfo {
    pub id: u32,
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "type")]
pub enum CellType {
    #[default]
    Default,
    Text(TextCell),
    #[serde(rename = "node")]
    Element(ElementCell),
    Root(Root),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataCell {
    pub id: u32,
    #[serde(flatten)]
    pub cell_type: CellType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextCell {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Prop {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ElementCell {
    #[serde(rename = "tag")]
    pub name: String,
    #[serde(rename = "attributes")]
    pub props: Vec<Prop>,
    pub children: Vec<DataCell>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Root {
    pub children: Vec<DataCell>,
}

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
                tag_stack_pop(&mut tag_stack, &indent);
                let res = self.result.clone();

                let default = &DataCell {
                    ..Default::default()
                };

                let curr_el_id = if let Some(last) = tag_stack.last() {
                    last.id
                } else {
                    0
                };
                println!("id {}", curr_el_id);

                let res_cloned = &mut self.result.clone();

                let current_cell: Option<&mut DataCell> = get_cell_by_id(res_cloned, curr_el_id);

                let el = init_element_cell(self.id, tag_name);

                if let Some(current_cell) = current_cell {
                    Self::add_cell(&mut self.result, curr_el_id, self.id, tag_name);
                }

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

    fn recursive(&mut self, cell: DataCell) {}

    fn add_cell(add_to: &mut DataCell, parent_id: u32, id: u32, tag_name: &str) {
        if add_to.id == parent_id {
            push_new_cell(add_to, id, tag_name);
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
