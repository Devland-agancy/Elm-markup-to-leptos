use regex::Regex;

use crate::parser_helpers::{BlockChildType, CellType, DataCell, DelimitedDisplayType};

use super::element_text::ElementText;
use super::helpers::*;
use std::str::Lines;

#[derive(Debug, Default)]
pub struct TagInfo {
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

#[derive(Debug)]
pub struct Emitter {
    pub self_closing_tags: Vec<&'static str>,
}

impl Emitter {
    pub fn new(self_closing_tags: Vec<&'static str>) -> Emitter {
        Emitter { self_closing_tags }
    }

    pub fn emit_json(&self, data_cell: &DataCell) -> String {
        let mut output = String::new();
        /* let json_tree: DataCell = serde_json::from_str(json).unwrap(); */

        match &data_cell.cell_type {
            CellType::Root(root) => {
                root.children.iter().for_each(|child| {
                    output.push_str(&self.emit_json(child));
                });
            }
            CellType::Element(el) => {
                output.push_str(&format!("<{} ", el.name));
                el.props.iter().for_each(|prop| {
                    output.push_str(&Self::handle_prop_line(&format!(
                        "{} {}",
                        prop.key, prop.value
                    )));
                });
                if self.self_closing_tags.contains(&el.name.as_str()) {
                    output.push_str(" />");
                } else {
                    output.push_str(" >");

                    el.children.iter().for_each(|child| {
                        output.push_str(&self.emit_json(child));
                    });
                    if el.children.is_empty() {
                        output.push_str("\"\"");
                    }
                    output.push_str(&format!("</{}>", el.name));
                }
            }
            CellType::Block(block) => {
                let mut text_block = String::new();
                let mut sub_text_block = String::new();

                block
                    .children
                    .iter()
                    .enumerate()
                    .for_each(|(i, child)| match child {
                        BlockChildType::Text(text) => {
                            if !text.content.trim().is_empty() {
                                sub_text_block.push_str(&format!("\"#<{}>r#\"", "span"));
                                sub_text_block.push_str(&text.content);
                                sub_text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                            } else {
                                sub_text_block.push_str(&text.content);
                            }
                        }
                        BlockChildType::Delimited(dl) => {
                            if let Some(wrap_with) = &dl.wrapped_with {
                                if !sub_text_block.trim().is_empty() {
                                    text_block
                                        .push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                                    text_block.push_str(&sub_text_block);
                                    text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                                }

                                sub_text_block = "".to_string();
                                text_block.push_str(&format!("\"#<{}>r#\"", wrap_with));
                                text_block.push_str(&dl.open_delimeter);
                                text_block.push_str(&dl.terminal);

                                text_block.push_str(&dl.close_delimeter);
                                text_block.push_str(&format!("\"#</{}>r#\"", wrap_with));
                            } else {
                                sub_text_block.push_str(&dl.open_delimeter);
                                sub_text_block.push_str(&dl.terminal);
                                sub_text_block.push_str(&dl.close_delimeter);
                            }
                        }
                    });
                if !sub_text_block.trim().is_empty() {
                    text_block.push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                    text_block.push_str(&sub_text_block);
                    text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                }

                let text_el = ElementText::new(&text_block);
                output.push_str(&format!("r#\"{}\"#", text_el.handle_delimeters()))
            }
            _ => {}
        }

        output.replace("r#\"\"#", "")
    }

    fn handle_prop_line(line: &str) -> String {
        let mut prop_line = line.trim().to_string();
        // add quotes to prop value if it's a string
        let prop_value = prop_line.split_once(" ");
        if let Some((prop_key, prop_value)) = prop_value {
            let is_number = prop_value.trim().parse::<f32>();
            let is_bool = prop_value.trim() == "false" || prop_value.trim() == "true";
            let is_vec = prop_value.trim().starts_with("vec![");

            if is_number.is_err() && !is_bool && !is_vec {
                prop_line = match prop_key {
                    "src" => format!(" {}=\"/{}\"", prop_key.trim(), prop_value.trim()),
                    _ => format!(" {}=\"{}\"", prop_key.trim(), prop_value.trim()),
                }
            } else {
                prop_line = format!(" {}={}", prop_key.trim(), prop_value.trim())
            }
        }
        prop_line
    }
}
