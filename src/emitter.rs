use leptos::html::{data, Data};
use regex::Regex;

use crate::counter::counter_commands::CommandType;
use crate::counter::counters::Counters;
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
pub struct Emitter<'a> {
    pub tree: &'a DataCell,
    pub self_closing_tags: Vec<&'a str>,
    text_to_attach_to_next_span: String,
    pub counters: &'a mut Counters,
}

impl<'a> Emitter<'a> {
    pub fn new(
        tree: &'a DataCell,
        self_closing_tags: Vec<&'a str>,
        counters: &'a mut Counters,
    ) -> Emitter<'a> {
        Emitter {
            tree,
            self_closing_tags,
            text_to_attach_to_next_span: String::new(),
            counters,
        }
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
                if el.is_counter {
                    let counter_type = CommandType::from_tag_name(&el.name).unwrap();
                    let counter_name = &el
                        .props
                        .iter()
                        .find(|c| c.key == "counter_name")
                        .unwrap()
                        .value;
                    let res = self.counters.execute(counter_type, counter_name);
                    if let Some(res) = res {
                        output.push_str(&format!("<span>\"{}\"</span>", res));
                    }
                } else {
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
            }
            CellType::Block(block) => {
                let mut text_block = String::new();
                let mut sub_text_block = String::new();
                // check if prev sibling cell is attached to this block
                let parent = DataCell::get_cell_by_id_immut(&self.tree, data_cell.parent_id);

                // if let Some(parent) = parent {
                //     let mut attached_elements_text = String::new();

                //     let prev_sibling = data_cell.get_prev_sibling(parent);
                //     let mut prev_sibling_loop = prev_sibling.clone();
                //     let mut last_sibling = prev_sibling.clone();

                //     while let Some(prev_sib) = prev_sibling_loop {
                //         if let CellType::Element(el) = &prev_sib.cell_type {
                //             attached_elements_text =
                //                 "value".to_owned() + attached_elements_text.as_str();
                //             prev_sibling_loop = prev_sib.get_prev_sibling(parent);
                //             last_sibling = prev_sib.get_prev_sibling(parent);

                //             if !el.is_attached_to_next && !el.is_attached_to_prev {
                //                 break;
                //             }
                //         } else {
                //             break;
                //         }
                //     }

                //     if let Some(prev_sibling) = &prev_sibling {
                //         if let CellType::Element(el) = &prev_sibling.cell_type {
                //             if (el.is_attached_to_next && !el.is_attached_to_prev)
                //                 || (el.is_attached_to_next && last_sibling.is_none())
                //             {
                //                 sub_text_block.push_str(&attached_elements_text)
                //             } else if el.is_attached_to_next && el.is_attached_to_prev {
                //             }
                //         }
                //     }
                // }

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
                                    let text_el = ElementText::new(sub_text_block.as_str());
                                    text_block.push_str(&text_el.handle_delimeters());
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

                let mut next_is_el_attached_to_prev = false;
                //Check if next cell is an attached element
                // if let Some(parent) = parent {
                //     let mut attached_elements_text = String::new();

                //     let next_sibling = data_cell.get_next_sibling(parent);
                //     let mut next_sibling_loop = next_sibling.clone();
                //     let mut last_sibling = next_sibling.clone();

                //     while let Some(next_sib) = next_sibling_loop {
                //         if let CellType::Element(el) = &next_sib.cell_type {
                //             if !el.is_attached_to_prev {
                //                 break;
                //             }

                //             attached_elements_text =
                //                 "value".to_owned() + attached_elements_text.as_str();
                //             next_sibling_loop = next_sib.get_next_sibling(parent);
                //             last_sibling = next_sib.get_next_sibling(parent);

                //             if el.is_attached_to_next {
                //                 self.text_to_attach_to_next_span = String::from(&sub_text_block);
                //                 self.text_to_attach_to_next_span
                //                     .push_str(&attached_elements_text);
                //                 next_is_el_attached_to_prev = true
                //             }
                //         } else {
                //             break;
                //         }
                //     }

                //     if let Some(_) = &next_sibling {
                //         sub_text_block.push_str(&attached_elements_text)
                //     }
                // }

                if !sub_text_block.trim().is_empty()
                // && !next_is_el_attached_to_prev
                // && self.text_to_attach_to_next_span.len() == 0
                {
                    text_block.push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                    let text_el = ElementText::new(sub_text_block.as_str());
                    text_block.push_str(&text_el.handle_delimeters());
                    text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                }

                // if !next_is_el_attached_to_prev && self.text_to_attach_to_next_span.len() > 0 {
                //     println!("last {:?}", self.text_to_attach_to_next_span);

                //     sub_text_block =
                //         self.text_to_attach_to_next_span.to_owned() + sub_text_block.as_str();
                //     self.text_to_attach_to_next_span = "".to_string();
                //     text_block.push_str(&format!("\"#<{} class=\"text\">r#\"", "span"));
                //     let text_el = ElementText::new(sub_text_block.as_str());
                //     text_block.push_str(&text_el.handle_delimeters());
                //     text_block.push_str(&format!("\"#</{}>r#\"", "span"));
                // }

                output.push_str(&format!("r#\"{}\"#", text_block));
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
