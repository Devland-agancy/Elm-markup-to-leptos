#[macro_use]
extern crate proc_macro;
extern crate nom;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, LitStr};

struct Input {
    cx: Ident,
    elm: LitStr,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cx: syn::Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let elm: LitStr = input.parse()?;
        Ok(Input { cx, elm })
    }
}

#[proc_macro]
pub fn elm_to_view(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;

    let leptos_code = trf(elm.value());
    let output = quote! {
        view! {
            cx, #leptos_code
        }
    };
    output.into()
}

struct TagInfo {
    name: String,
    indent: usize,
    is_self_closing: bool,
    in_props: bool,
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

    pub fn add_delimeter(
        mut self,
        symbol: &str,
        left_replacement: &str,
        right_replacement: &str,
        no_break: bool,
        keep_delimeter: bool,
    ) -> Self {
        self.text = ContentLine::escape_chars(&self.text, symbol);

        let re = regex::Regex::new(&format!(
            r"\{}(.*?)\{}{}",
            symbol,
            symbol,
            if no_break { "(\\S*)" } else { "" }
        ))
        .unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                if caps.get(2).is_some() && caps.get(2).unwrap().len() > 0 {
                    // If the character after the second $ is not a space
                    format!(
                        "\"#<span class=\"nobreak\">{}r#\"{}{}{}\"#{}\"{}\"</span>r#\"",
                        left_replacement,
                        if keep_delimeter { symbol } else { "" },
                        &caps[1],
                        if keep_delimeter { symbol } else { "" },
                        right_replacement,
                        &caps[2]
                    )
                } else {
                    format!(
                        "\"#{}r#\"{}{}{}\"#{}r#\"",
                        left_replacement,
                        if keep_delimeter { symbol } else { "" },
                        &caps[1],
                        if keep_delimeter { symbol } else { "" },
                        right_replacement
                    )
                }
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, symbol);

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

fn trf(elm: String) -> proc_macro2::TokenStream {
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

                let processed_text = ContentLine::new(&text_node)
                    .add_delimeter("*", "<Span bold=true>", "</Span>", false, false)
                    .add_delimeter(
                        "_\\_",
                        "<Paragragh Align::Center><Span italic=true>",
                        "</Span></Paragraph>",
                        false,
                        false,
                    )
                    .add_delimeter("_", "<Span italic=true>", "</Span>", false, false)
                    .add_delimeter("$\\$", "<MathBlock>", "</MathBlock>", true, false)
                    .add_delimeter("$", "<Math>", "</Math>", true, true)
                    .handle_math_block_back();

                output.push_str(&concat_ignore_spaces("r#\"", &processed_text.text, "\"#\n"));
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

    output
        .replace("\n", " ")
        .parse::<proc_macro2::TokenStream>()
        .expect("Failed to parse Leptos view code")
}
