use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    fn handle_math(mut self) -> Self {
        let re = regex::Regex::new(r"\$(.*?)\$").unwrap();

        for cap in re.captures_iter(&self.text.clone()) {
            let match_str = &cap[0];
            let start_pos = cap.get(0).unwrap().start();
            let end_pos = cap.get(0).unwrap().end();

            // Check if the match is not part of a $$ sequence
            if (start_pos == 0 || self.text.chars().nth(start_pos - 1).unwrap() != '$')
                && (end_pos >= self.text.len() || self.text.chars().nth(end_pos).unwrap() != '$')
            {
                self.text = re
                    .replace_all(&self.text, |caps: &regex::Captures| {
                        format!("\"#<Math>r#\"${}$\"#</Math>r#\"", &caps[1])
                    })
                    .to_string();
            }
        }

        self
    }

    fn handle_math_block(mut self) -> Self {
        let re = regex::Regex::new(r"\$\$(.*?)\$\$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<MathBlock>r#\"$${}$$\"#</MathBlock>r#\"", &caps[1])
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
fn trf(elm: &str) -> String {
    let path = Path::new(elm);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let mut output = String::new();
            let mut tag_stack: Vec<TagInfo> = Vec::new();
            let lines = s.lines();

            for line in lines {
                let trimmed_line = line.trim();
                let indent = line.len() - trimmed_line.len();

                if trimmed_line.starts_with("|> ") {
                    let tag_name = trimmed_line[3..].to_string();
                    tag_loop(&mut tag_stack, &mut output, &indent);

                    output.push_str(&format!("<{}\n", tag_name));
                    tag_stack.push(TagInfo {
                        name: tag_name,
                        indent,
                        is_self_closing: false,
                        in_props: true,
                    });
                } else if trimmed_line.starts_with("|>_ ") {
                    let tag_name = trimmed_line[4..].to_string();
                    tag_loop(&mut tag_stack, &mut output, &indent);

                    output.push_str(&format!("<{} \n", tag_name));
                    tag_stack.push(TagInfo {
                        name: tag_name,
                        indent,
                        is_self_closing: true,
                        in_props: true,
                    });
                } else if trimmed_line.is_empty()
                    && tag_stack.last().map_or(false, |tag| tag.in_props)
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
                            .handle_math();
                        output.push_str(&concat_ignore_spaces(
                            "r#\"",
                            &processed_line.text,
                            "\"#\n",
                        ));
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
    }
}

fn main() {
    let html_code = trf("/home/chaker/code/lp/Elm-markup-to-leptos/src/elm.emu");
    println!("{}", html_code);
}
