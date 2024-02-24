use leptos::html::{Br, Tr};

use super::element_text::ElementText;
use std::{collections::btree_map::Range, iter::Enumerate, str::Lines};

#[derive(Debug)]
struct TagInfo {
    name: String,
    indent: usize,
    is_self_closing: bool,
    in_props: bool,
}

#[derive(Debug)]
pub struct Transformer {
    pub self_closing_tags: Vec<&'static str>,
    pub tags_with_paragraphs: Vec<&'static str>,
    pub paragraph_tag: &'static str,
}

impl Transformer {
    pub fn new(
        self_closing_tags: Vec<&'static str>,
        tags_with_paragraphs: Vec<&'static str>,
        paragraph_tag: &'static str,
    ) -> Transformer {
        Transformer {
            self_closing_tags,
            paragraph_tag,
            tags_with_paragraphs,
        }
    }

    pub fn transform(&self, elm: String, start_index: usize) -> String {
        let mut output = String::new();
        let mut tag_stack: Vec<TagInfo> = Vec::new();
        let lines = elm.lines();
        let mut lines_to_skip: u32 = 0;
        for (index, line) in lines.clone().enumerate() {
            if lines_to_skip > 0 {
                lines_to_skip = lines_to_skip - 1;
                continue;
            }

            let trimmed_line = line.trim_start();
            let indent = Self::get_line_indent(line, trimmed_line);
            Self::check_indent_size(indent, index);

            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].to_string();
                Self::tag_loop(&mut tag_stack, &mut output, &indent);
                output.push_str(&format!("<{}\n", tag_name));
                tag_stack.push(TagInfo {
                    name: tag_name,
                    indent,
                    is_self_closing: self
                        .self_closing_tags
                        .contains(&&trimmed_line[3..].trim_end()),
                    in_props: true,
                });
                continue;
            }
            if trimmed_line.is_empty() // end of props
          && tag_stack
              .last()
              .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
            {
                output.push_str(">\n\"\" ");
                if let Some(last) = tag_stack.last_mut() {
                    last.in_props = false;
                }
                continue;
            }
            if !trimmed_line.is_empty() {
                Self::tag_loop(&mut tag_stack, &mut output, &indent);

                let last = tag_stack.last().expect(&format!(
                    "There is no parent tag . line {}",
                    index + start_index
                ));
                if last.in_props {
                    // tag props
                    output.push_str(&format!("{}\n", line));
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
                    let indent = Self::get_line_indent(text_line, inner_trimmed_line);
                    Self::check_indent_size(indent, index);

                    // break if next line is new ( not nested ) element
                    if let Some(next_line) = lines.clone().nth(index + j + 1) {
                        let next_line_trimmed = next_line.trim_start();
                        let next_line_indent = Self::get_line_indent(next_line, next_line_trimmed);
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
                        self.handle_inline_element_1(lines.clone(), j + index, indent);

                    inner_lines_to_skip += skips;
                    lines_to_skip += skips + 1;

                    nodes
                        .last_mut()
                        .unwrap()
                        .push((format!("\"#{}r#\"", element), true));
                }

                let mut processed_text = String::new();

                for sub_nodes in nodes {
                    if sub_nodes.len() == 0 {
                        continue;
                    }
                    // subnodes are seperated with empty line
                    let mut sub_node_text = "".to_string();

                    for (text, is_element) in sub_nodes {
                        if is_element {
                            sub_node_text += &text;
                            continue;
                        }
                        sub_node_text += &ElementText::new(&text).handle_delimeters();
                    }

                    if self
                        .tags_with_paragraphs
                        .contains(&tag_stack.last().unwrap().name.as_str())
                    {
                        processed_text += &format!(
                            "\"#<{}>r#\"{}\"#</{}>r#\"",
                            self.paragraph_tag, sub_node_text, self.paragraph_tag
                        );
                        continue;
                    }
                    processed_text += sub_node_text.as_str();
                }
                nodes = vec![];
                output.push_str(&Self::concat_ignore_spaces(
                    "r#\"",
                    &processed_text,
                    "\"#\n",
                ));
            }
        }

        while let Some(last_tag_info) = tag_stack.pop() {
            if last_tag_info.is_self_closing {
                output.push_str("/>\n");
                continue;
            }
            output.push_str(&format!("</{}>\n", last_tag_info.name));
        }

        output.replace("\n", " ")
    }

    fn handle_inline_element_1(
        &self,
        lines: Lines<'_>,
        start_from: usize,
        initial_indent: usize,
    ) -> (String, u32) {
        let mut element = "".to_string();
        let mut end_line = start_from;
        let mut inner_indent = initial_indent + 4;
        let mut skips: u32 = 0;
        let mut prev_line = "";

        while inner_indent > initial_indent || inner_indent == 0 {
            if lines.clone().nth(end_line + 1).is_none() {
                break;
            }

            let inner_line = lines.clone().nth(end_line + 1).unwrap();
            let inner_trimmed_line = inner_line.trim_start();
            inner_indent = Self::get_line_indent(inner_line, inner_trimmed_line);

            if inner_indent > initial_indent || inner_indent == 0 {
                end_line += 1;
                skips += 1;
            }
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
        element = self.transform(element, start_from);
        (element, skips)
    }

    fn concat_ignore_spaces(start: &str, content: &str, end: &str) -> String {
        let trimmed_content = content.trim_start(); // Remove leading spaces from content
        format!("{}{}{}", start, trimmed_content, end)
    }

    fn tag_loop(tag_stack: &mut Vec<TagInfo>, output: &mut String, indent: &usize) {
        while let Some(last_tag_info) = tag_stack.last() {
            if *indent <= last_tag_info.indent {
                if last_tag_info.is_self_closing {
                    output.push_str("/>\n");
                } else {
                    output.push_str(&format!("</{}>\n", last_tag_info.name));
                }
                tag_stack.pop();
            } else {
                break;
            }
        }
    }

    fn get_line_indent(line: &str, trimmed: &str) -> usize {
        if line.is_empty() || line.chars().all(char::is_whitespace) {
            return 0;
        };
        line.len() - trimmed.len()
    }

    fn check_indent_size(size: usize, error_at: usize) {
        if size % 4 != 0 {
            panic!(
                "Syntax error at line {}, There must be 4 spaces before each block",
                error_at
            )
        }
    }
}
