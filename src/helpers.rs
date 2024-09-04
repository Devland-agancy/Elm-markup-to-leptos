use crate::emitter::{Emitter, TagInfo};

pub fn concat_ignore_spaces(start: &str, content: &str, end: &str) -> String {
    let trimmed_content = content.trim_start(); // Remove leading spaces from content
    format!("{}{}{}", start, trimmed_content, end)
}

pub fn get_tag_from_string(string: &str) -> String {
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

pub fn tag_loop(tag_stack: &mut Vec<TagInfo>, output: &mut String, indent: &usize) {
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

pub fn get_line_indent(line: &str) -> usize {
    if line.is_empty() || line.chars().all(char::is_whitespace) {
        return 0;
    };
    line.len() - line.trim_start().len()
}

pub fn check_indent_size(size: isize, line: usize) {
    if size % 4 != 0 {
        panic!(
            "Syntax error: \\n
            There must be 4*x spaces before line {}",
            line
        )
    }
}

pub fn check_extra_spaces(indent: usize, parent_indent: usize, line: usize) {
    if indent > parent_indent + 4 {
        panic!(
            "Syntax error: \\n
            There are extra spaces at line {}",
            line
        )
    }
}

pub fn get_slice(text: &str, start: usize, end: usize) -> Option<&str> {
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

//pub fn track_emitter_line_diff(parser: &mut Emitter, tag_name: &String) {
//    if tag_name == "Exercises" {
//        parser.track_line_delta += 1
//    }
//
//if tag_name         == "Example" {
//        parser.track_line_delta += 1
//    }
//
//    if tag_name == "Exercise" {
//        parser.track_line_delta += 3
//    }
//
//    if tag_name == "Solution" {
//        parser.track_line_delta += 1
//    }
//}
