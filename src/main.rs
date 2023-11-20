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

    fn handle_bold(mut self) -> Self {
        let re = regex::Regex::new(r"\*(.*?)\*").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span bold=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();
        self
    }

    fn handle_italic(mut self) -> Self {
        let re = regex::Regex::new(r"\%(.*?)\%").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span italic=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();
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
        let re = regex::Regex::new(r"\$(.*?)\$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Math>r#\"${}$\"#</Math>r#\"", &caps[1])
            })
            .to_string();
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
    let self_closing_tags = vec!["Image"];
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
                is_self_closing: self_closing_tags.contains(&&trimmed_line[3..]),
                in_props: true,
            });
        } else if trimmed_line.is_empty()
            && tag_stack
                .last()
                .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
        {
            output.push_str(">\n");
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
    |> Paragraph

        *The Definition.*
        The %slope% of a line is a mathematical measure of how
        &ldquo;steep&rdquo; a line is.
        Here are a few examples (for an explanation of the values,
        see below):

    |> Image
        src="ch2figs/collection_of_examples_3_x_2.svg"

    |> Paragraph    
        
        The slope of a line is...

    |> Paragraph   
        margin_top = 15

        the number of units the line goes up with each unit to the right
        
    |> Paragraph
        margin_top = 15

        ...assuming that numbers on the $y$-axis increase going up and that 
        numbers on the $x$-axis increase going right, as is usually the case.
        One can also describe slope as...
    "#
    .to_string());
    println!("{}", html_code);
}
