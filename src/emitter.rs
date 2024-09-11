use std::collections::HashMap;

use super::element_text::ElementText;
use crate::{
    counter::counter_types::CounterType,
    datacell::{BlockChildType::*, Datacell::*},
};

#[derive(Debug, Default)]
pub struct TagInfo {
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

#[derive(Debug)]
pub struct Emitter<'a> {
    pub self_closing_tags: Vec<&'a str>,
}

impl<'a> Emitter<'a> {
    pub fn new(self_closing_tags: Vec<&'a str>) -> Emitter<'a> {
        Emitter { self_closing_tags }
    }

    pub fn emit_json(&mut self, data_cell: &DataCell) -> String {
        let mut output = String::new();

        match &data_cell.cell_type {
            CellType::Root(root) => {
                root.children.iter().for_each(|child| {
                    output.push_str(&self.emit_json(child));
                });
            }
            CellType::Element(el) => {
                output.push_str(&format!("<{} ", el.name));
                el.props.iter().for_each(|prop| {
                    if !CounterType::is_valid(&prop.key) {
                        output.push_str(&Self::handle_prop_line(&format!(
                            "{} {}",
                            prop.key, prop.value
                        )));
                    }
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
                    .for_each(|(_, child)| match child {
                        BlockChildType::Text(text) => {
                            sub_text_block.push_str(&text.content);
                        }
                        BlockChildType::Delimited(dl) => {
                            if let Some(_) = &dl.wrapped_with {
                                if !sub_text_block.trim().is_empty() {
                                    text_block
                                        .push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                                    let mut text_el = ElementText::new(sub_text_block.as_str());
                                    text_block
                                        .push_str(&&text_el.remove_escapes().handle_delimeters());
                                    text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                                }
                                sub_text_block = "".to_string();
                                //text_block.push_str(&format!("\"#<{}>r#\"", wrap_with));

                                sub_text_block.push_str(&dl.open_delimeter);
                                sub_text_block.push_str(&dl.terminal);
                                sub_text_block.push_str(&dl.close_delimeter);

                                let text_el = ElementText::new(sub_text_block.as_str());
                                text_block.push_str(&text_el.handle_delimeters());
                                //text_block.push_str(&format!("\"#</{}>r#\"", wrap_with));
                                sub_text_block = "".to_string();
                            } else if dl.display_type == DelimitedDisplayType::BLOCK {
                                sub_text_block = "".to_string();
                                sub_text_block.push_str(&dl.open_delimeter);
                                sub_text_block.push_str(&dl.terminal);
                                sub_text_block.push_str(&dl.close_delimeter);
                                let text_el = ElementText::new(sub_text_block.as_str());
                                text_block.push_str(&text_el.handle_delimeters());
                                sub_text_block = "".to_string();
                            } else {
                                sub_text_block.push_str(&dl.open_delimeter);
                                sub_text_block.push_str(&dl.terminal);
                                sub_text_block.push_str(&dl.close_delimeter);
                            }
                        }
                    });

                if !sub_text_block.trim().is_empty() {
                    text_block.push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                    let text_el = ElementText::new(sub_text_block.as_str());
                    text_block.push_str(&text_el.handle_delimeters());
                    text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                }

                output.push_str(&format!("r#\"{}\"#", text_block));
            }
            _ => {}
        }

        output.replace("r#\"\"#", "")
    }

    pub fn split_and_emit(
        &mut self,
        data_cell: &DataCell,
        split_children_of: &str,
    ) -> HashMap<String, String> {
        let mut output = HashMap::new();
        match &data_cell.cell_type {
            CellType::Element(el) => {
                if el.name.contains(&split_children_of) {
                    el.children.iter().for_each(|child| {
                        if let CellType::Element(child_el) = &child.cell_type {
                            output.insert(
                                child_el.name.clone().to_lowercase(),
                                self.emit_json(child),
                            );
                        }
                    });
                } else {
                    el.children.iter().for_each(|child| {
                        output.extend(self.split_and_emit(child, split_children_of));
                    });
                }
            }
            CellType::Root(root) => {
                root.children.iter().for_each(|child| {
                    output.extend(self.split_and_emit(child, split_children_of));
                });
            }
            _ => (),
        }
        output
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
