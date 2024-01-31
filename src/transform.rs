use super::element_text::ElementText;
use std::str::Lines;

#[derive(Debug)]
struct TagInfo {
    name: String,
    indent: usize,
    is_self_closing: bool,
    in_props: bool,
}

pub fn transform(elm: String) -> String {
    let self_closing_tags = vec!["Image", "img", "SectionDivider"];

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
        let indent = line.len() - trimmed_line.len();
        check_indent_size(indent, index);

        if trimmed_line.starts_with("|> ") {
            let tag_name = trimmed_line[3..].to_string();
            tag_loop(&mut tag_stack, &mut output, &indent);
            output.push_str(&format!("<{}\n", tag_name));
            tag_stack.push(TagInfo {
                name: tag_name,
                indent,
                is_self_closing: self_closing_tags.contains(&&trimmed_line[3..].trim_end()),
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
            tag_loop(&mut tag_stack, &mut output, &indent);
            let last = tag_stack.last().expect("There is no parent tag");
            if last.in_props {
                // tag props
                output.push_str(&format!("{}\n", line));
                continue;
            }
            // tag content
            let mut nodes: Vec<(String, bool)> = Vec::new(); // if there is an inline element in the text , the text should be seperated to sub text before the element , the element ( bool is added to know if the node is an element ) and the subtext after the element . it shouldn't be handeled as 1 text
            let mut text_node = String::new();
            let mut inner_lines_to_skip: u32 = 0;

            for (j, text_line) in lines.clone().skip(index).enumerate() {
                if inner_lines_to_skip > 0 {
                    inner_lines_to_skip = inner_lines_to_skip - 1;
                    continue;
                }
                let trimmed_line = text_line.trim_start();
                let indent = text_line.len() - trimmed_line.len();
                check_indent_size(indent, index);
                if trimmed_line.is_empty() {
                    break;
                }
                if !trimmed_line.starts_with("|>") {
                    text_node += &format!(" {}", trimmed_line);
                    lines_to_skip += 1;
                    continue;
                }
                nodes.push((text_node, false));
                text_node = "".to_string();
                // handle inline element that can't be handled by delimiters ( they need props )
                let (element, skips) =
                    handle_inline_element(lines.clone(), trimmed_line, j + index + 1, indent);
                inner_lines_to_skip += skips;
                lines_to_skip += skips;
                nodes.push((format!("\"#{}r#\"", element), true));
            }
            if text_node != "" {
                nodes.push((text_node, false));
            }
            let mut processed_text = String::new();
            for (text, is_element) in nodes {
                if is_element {
                    processed_text += &text;
                    continue;
                }
                processed_text += &ElementText::new(&text).handle_delimeters();
            }
            output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n"));
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

fn handle_inline_element(
    lines: Lines<'_>,
    element_line: &str,
    start_from: usize,
    initial_indent: usize,
) -> (String, u32) {
    let mut element = "".to_string();
    let mut element_content = "".to_string();
    let mut end_line = start_from;
    let mut inner_indent = initial_indent + 4;
    let mut skips: u32 = 0;
    let mut in_props = true;

    let tag_name = element_line[3..].to_string();
    element.push_str(&format!("<{} \n", tag_name));
    while inner_indent > initial_indent || inner_indent == 0 {
        if lines.clone().nth(end_line).is_none() {
            break;
        }
        let inner_line = lines.clone().nth(end_line).unwrap();
        let inner_trimmed_line = inner_line.trim_start();
        inner_indent = inner_line.len() - inner_trimmed_line.len();
        end_line += 1;
        if inner_indent == 0 && in_props {
            element.push_str(">\n r#\"");
            skips += 1;
            in_props = false;
            continue;
        }
        if inner_indent > initial_indent {
            skips += 1;
            if in_props {
                element.push_str(&(inner_trimmed_line.to_string() + " "));
                continue;
            }
            element_content.push_str(&(inner_trimmed_line.to_string() + " "));
        }
    }

    element.push_str(&ElementText::new(&element_content).handle_delimeters());
    element.push_str(&format!("\"#</{}>\n", tag_name));
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

fn check_indent_size(size: usize, error_at: usize) {
    if size % 4 != 0 {
        panic!(
            "Syntax error at line {}, There must be 4 spaces before each block",
            error_at
        )
    }
}
