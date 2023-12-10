#[macro_use]
extern crate proc_macro;
extern crate nom;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use nom::{
    character::complete::{char, multispace0, none_of},
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};
use regex::Regex;

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
    /* let tokens: proc_macro2::TokenStream = input.clone().into();
    let mut tokens = tokens.into_iter();
    let (cx, content) = (tokens.next(), tokens.next()); */
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;

    let leptos_code = trf(elm.value());
    /*  println!(
        "**********************************{}******************************************",
        leptos_code
    ); */
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
        self.text = ContentLine::escape_chars(&self.text, "%");

        let re = regex::Regex::new(r"\%(.*?)\%").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span italic=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "%");

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

        let re = regex::Regex::new(r"\$(.*?)\$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Math>r#\"${}$\"#</Math>r#\"", &caps[1])
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
fn trf(elm: String) -> proc_macro2::TokenStream {
    let mut output = String::new();
    let mut tag_stack: Vec<TagInfo> = Vec::new();
    let lines = elm.lines();
    let self_closing_tags = vec!["Image", "img"];

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

                output.push_str(&concat_ignore_spaces(
                    "r#\"",
                    &processed_line.text,
                    " \"#\n",
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

    output
        .replace("\n", " ")
        .parse::<proc_macro2::TokenStream>()
        .expect("Failed to parse Leptos view code")
}
