pub mod utils;
use crate::utils::ContentLine::ContentLine;

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
        if indent % 4 != 0 {
            panic!(
                "Syntax error at line {}, There must be 4 spaces before each block",
                index
            )
        }
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
        } else if trimmed_line.is_empty() // props lines
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

            let last = tag_stack.last().expect("There is no parent tag");
            if last.in_props {
                output.push_str(&format!("{}\n", line));
            } else {
                let mut text_node = String::new();
                for (j, text_line) in lines.clone().skip(index).enumerate() {
                    let trimmed_line = text_line.trim_start();
                    let indent = text_line.len() - trimmed_line.len();
                    if indent % 4 != 0 {
                        panic!(
                            "Syntax error at line {}, There must be 4 spaces before each block",
                            j + index
                        )
                    }

                    if text_line.trim_start().is_empty() {
                        break;
                    } else {
                        text_node += &format!(" {}", text_line.trim_start());
                        lines_to_skip = lines_to_skip + 1
                    }
                }

                let processed_text = ContentLine::new(&text_node).handle_delimeters();

                output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n"));
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
    margin_top = 15 
    
    On the other hand, an equation of the form
    $$
    y = ax + b
    $$
    cannot describe a vertical line, because $a$ is the slope, while<br>
    a vertical line has no slope, so what would $a$ be equal to?
    Instead, a vertical line is described by an equation of the form
    $$
    x = c
    $$
    (see Fig. 1) where $c \in \rr$ is a constant,
    similarly to the more familiar equation
    $$
    y = b
    $$
    for a horizontal line, where $b \in \rr$ is a constant.

    |> ImageLeft
        src="/images/50.svg"
        width=200
        pos_y="-10.5rem"
        pos_x="1rem"
        squiggle_right="0rem"
        squiggle_top="62%"
        hidden_in_mobile=true

|> Paragraph   
    indent=Indent::Line

    One should also keep in mind that an equation can define a
    line without having either of the forms “$y = ax + b$”
    or “$x = c$”. For example,
    $$
    x + y = 3
    $$

|> Paragraph   
    margin_top = 15

    is equivalent to
    $$
    y = 3 - x
    $$

|> Paragraph   
    margin_top = 15

    and thus describes a line of $y$-intercept $3$ and slope $-1$.
    "#
    .to_string());
    println!("{}", html_code);
}
