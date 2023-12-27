use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use nom::sequence::Tuple;
#[derive(Debug)]
struct TagInfo {
    name: String,
    indent: usize,
    is_self_closing: bool,
    in_props: bool,
}

fn concat_ignore_spaces(start: &str, content: &str, end: &str) -> String {
    let trimmed_content = content.trim_start(); // Remove leading spaces from content
    format!("{}{}{}", start, trimmed_content, end)
}

struct ContentLine {
    text: String,
}
impl ContentLine {
    fn new(text: &str) -> ContentLine {
        ContentLine {
            text: text.to_string(),
        }
    }
    //fn replace_symbols(mut self, symbol: &str, replacement_start: &str, replacement_end: &str) -> Self {
    //    let re = regex::Regex::new(&format!(r"\{}(.*?)\{}", symbol, symbol)).unwrap();
    //    self.text = re
    //        .replace_all(&self.text, |caps: &regex::Captures| {
    //            format!("\"#{}r#\"{}\"#{}r#\"", replacement_start,  &caps[1], replacement_end);
    //        })
    //        .to_string();
    //    self
    //}

    fn handle_bold(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "*");

        let re = regex::Regex::new(r"\*(.*?)\*").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span bold=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "*");

        self
    }

    fn handle_italic(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "_");

        let re = regex::Regex::new(r"\_(.*?)\_").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span italic=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "_");

        self
    }

    fn handle_math_block(mut self) -> Self {
        let re = regex::Regex::new(r"\$\$(.*?)\$\$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                // $$ is not included here as it will cause issues with handle_math fn , after we call handle_math we add $$ back
                format!("\"#<MathBlock>{}</MathBlock>r#\"", &caps[1])
            })
            .to_string();
        self
    }

    fn handle_math(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "$");

        let re = regex::Regex::new(r"\$(.*?)\$(\S*)").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                if caps.get(2).is_some() && caps.get(2).unwrap().len() > 0 {
                    // If the character after the second $ is not a space
                    format!(
                        "\"#<span><Math>r#\"${}$\"#</Math>\"{}\"</span>r#\"",
                        &caps[1], &caps[2]
                    )
                } else {
                    format!("\"#<Math>r#\"${}$\"#</Math>r#\"", &caps[1])
                }
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "$");

        self
    }

    fn handle_math_block_back(mut self) -> Self {
        let re = regex::Regex::new(r"<MathBlock>(.*?)</MathBlock>").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("<MathBlock>r#\"$${}$$\"#</MathBlock>", &caps[1])
            })
            .to_string();
        self
    }

    fn escape_chars(text: &str, _char: &str) -> String {
        // handle escaped $
        let re = regex::Regex::new(&format!(r"\\\{}(.*?)\\\{}", _char, _char)).unwrap();
        let res = re
            .replace_all(&text, |caps: &regex::Captures| {
                format!("XescapedX{}XescapedX", &caps[1])
            })
            .to_string();

        res
    }
    fn un_escape_chars(text: &str, _char: &str) -> String {
        let re = regex::Regex::new(r"XescapedX(.*?)XescapedX").unwrap();
        let res = re
            .replace_all(&text, |caps: &regex::Captures| {
                format!("{}{}{}", _char, &caps[1], _char)
            })
            .to_string();

        res
    }
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

// Let's also update the `trf` function to use `TagInfo`:
fn trf(elm: String) -> String {
    let self_closing_tags = vec!["Image", "img"];
    let mut s: String = String::new();

    let mut output = String::new();
    let mut tag_stack: Vec<TagInfo> = Vec::new();
    let lines = elm.lines();
    for line in lines {
        let trimmed_line = line.trim_start();
        let indent = line.len() - trimmed_line.len();

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
        } else if trimmed_line.is_empty()
            && tag_stack
                .last()
                .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
        {
            output.push_str(">\n\"\" ");
            if let Some(last) = tag_stack.last_mut() {
                last.in_props = false;
            }
        } else if !trimmed_line.is_empty() {
            tag_loop(&mut tag_stack, &mut output, &indent);

            let last = tag_stack.last().unwrap();
            if last.in_props {
                output.push_str(&format!("{}\n", line));
            } else {
                let processed_line = ContentLine::new(line)
                    .handle_bold()
                    .handle_italic()
                    .handle_math_block()
                    .handle_math()
                    .handle_math_block_back();

                output.push_str(&concat_ignore_spaces("r#\"", &processed_line.text, "\"#\n"));
            }
        }
    }

    while let Some(last_tag_info) = tag_stack.pop() {
        if last_tag_info.is_self_closing {
            output.push_str("/>\n");
        } else {
            output.push_str(&format!("</{}>\n", last_tag_info.name));
        }
    }

    output.replace("\n", " ")
}

fn main() {
    let html_code = trf(r#"
|> PRR 
    src="/images/33.svg"

    $math$ hihi $other$w wowo 
    $other$wsd d

    "#
    .to_string());
    println!("{}", html_code);
}
