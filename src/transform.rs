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
    pub tags_before_non_indents: Vec<&'static str>,
    pub no_x_padding_tags: Vec<&'static str>,
    pub tags_with_non_indent_first_child: Vec<&'static str>,
}

impl Transformer {
    pub fn new(
        self_closing_tags: Vec<&'static str>,
        tags_with_paragraphs: Vec<&'static str>,
        no_x_padding_tags: Vec<&'static str>,
        paragraph_tag: &'static str,
        tags_before_non_indents: Vec<&'static str>,
        tags_with_non_indent_first_child: Vec<&'static str>,
    ) -> Transformer {
        Transformer {
            self_closing_tags,
            paragraph_tag,
            tags_with_paragraphs,
            tags_with_non_indent_first_child,
            tags_before_non_indents,
            no_x_padding_tags,
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
            Self::check_indent_size(indent, index + start_index);
            if let Some(last) = tag_stack.last() {
                Self::check_extra_spaces(indent, last.indent, index + start_index);
            }
            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].trim().to_string();
                Self::tag_loop(&mut tag_stack, &mut output, &indent);
                output.push_str(&format!("<{}\n", tag_name));
                tag_stack.push(TagInfo {
                    name: tag_name.clone(),
                    indent,
                    is_self_closing: self.self_closing_tags.contains(&tag_name.as_str()),
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
                    Self::check_indent_size(indent, index + start_index + j);
                    Self::check_extra_spaces(
                        indent,
                        tag_stack.last().unwrap().indent,
                        index + start_index + j,
                    );
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
                        self.handle_inline_element_1(lines.clone(), j + index, indent);

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

                        let no_paragraph_indent = self.handle_paragraph_indent(
                            &tag_stack,
                            &nodes,
                            &sub_nodes,
                            node_idx,
                            sub_node_idx,
                            &text,
                        );

                        if no_paragraph_indent {
                            sub_node_text += &ElementText::new(&text).handle_delimeters();
                        } else {
                            let handled_text = &format!(
                                "\"#<Indent>r#\"{}\"#</Indent>r#\"",
                                &ElementText::new(&text).handle_delimeters()
                            );
                            sub_node_text += handled_text;
                        }
                    }

                    if self
                        .tags_with_paragraphs
                        .contains(&tag_stack.last().unwrap().name.as_str())
                    {
                        let no_padding = self
                            .no_x_padding_tags
                            .contains(&tag_stack.last().unwrap().name.as_str());

                        processed_text += &format!(
                            "\"#<{} {}>r#\"{}\"#</{}>r#\"",
                            self.paragraph_tag,
                            if no_padding { "no_padding = true" } else { "" },
                            sub_node_text,
                            self.paragraph_tag
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

        output
            .replace("\n", " ")
            .replace("r#\"\"#", "")
            .replace("r#\" \"#", " ")
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
            inner_indent = Self::get_line_indent(inner_line, inner_trimmed_line);

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
        element = self.transform(element, start_from);
        (element, skips)
    }

    fn concat_ignore_spaces(start: &str, content: &str, end: &str) -> String {
        let trimmed_content = content.trim_start(); // Remove leading spaces from content
        format!("{}{}{}", start, trimmed_content, end)
    }

    fn get_tag_from_string(string: &str) -> String {
        let mut i = 3;
        let mut tag = "".to_string();
        while i < string.len()
            && string.chars().nth(i).unwrap() != ' '
            && string.chars().nth(i).unwrap() != '>'
        {
            tag.push(string.chars().nth(i).unwrap());
            i += 1
        }
        tag
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
                error_at + 1
            )
        }
    }

    fn check_extra_spaces(indent: usize, parent_indent: usize, error_at: usize) {
        if indent > parent_indent + 4 {
            panic!(
                "Syntax error at line {}, There are extra spaces",
                error_at + 1
            )
        }
    }

    fn get_slice(text: &str, start: usize, end: usize) -> Option<&str> {
        assert!(end >= start);

        let mut iter = text
            .char_indices()
            .map(|(pos, _)| pos)
            .chain(Some(text.len()))
            .skip(start)
            .peekable();
        let start_pos = *iter.peek()?;
        for _ in start..end {
            iter.next();
        }

        Some(&text[start_pos..*iter.peek()?])
    }
    fn handle_paragraph_indent(
        &self,
        tag_stack: &Vec<TagInfo>,
        nodes: &Vec<Vec<(String, bool)>>,
        sub_nodes: &Vec<(String, bool)>,
        node_idx: usize,
        sub_node_idx: usize,
        text: &str,
    ) -> bool {
        // For |> Indent the emitter’s rule is that a paragraph has an indent by default except:
        let mut no_paragraph_indent = false;
        // the first paragraph of every section, exercise, or example does not have an indent

        no_paragraph_indent = node_idx == 0
            && sub_node_idx == 0
            && self
                .tags_with_non_indent_first_child
                .contains(&tag_stack.last().unwrap().name.as_str());
        // paragraph starting with _
        no_paragraph_indent = no_paragraph_indent
            || (text.len() > 2 && ["_"].contains(&&Self::get_slice(text, 1, 2).unwrap()));

        // $$, __,  |_ paragraphs
        let centering_delimiters = vec!["$$", "__", "|_"];
        no_paragraph_indent = no_paragraph_indent
            || (text.len() > 4
                && centering_delimiters.contains(&&Self::get_slice(text, 1, 3).unwrap()));

        // a paragraph that follows a paragraph ending with the $$, __,  |_ delimeter does not have an indent
        if !no_paragraph_indent && node_idx > 0 {
            if let Some(last_prev_sub_node) = nodes[node_idx - 1].last() {
                let prev_text = &last_prev_sub_node.0;
                if prev_text.len() > 1 {
                    no_paragraph_indent =
                        centering_delimiters.contains(&&prev_text[prev_text.len() - 2..]);
                }
            }
        }
        // a paragraph that follows tags defined in tags_before_non_indents
        if !no_paragraph_indent {
            let mut prev_tag = "".to_string();
            if sub_node_idx > 0 {
                prev_tag = Self::get_tag_from_string(sub_nodes[sub_node_idx - 1].0.as_str());
            }
            if node_idx > 0 {
                prev_tag =
                    Self::get_tag_from_string(nodes[node_idx - 1].last().unwrap().0.as_str());
            }
            no_paragraph_indent = self.tags_before_non_indents.contains(&prev_tag.as_str());
        }
        no_paragraph_indent
    }
}
