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

    let leptos_code = trf(&elm.value());
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
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Math>r#$\"{}$\"#</Math>r#\"", &caps[1])
            })
            .to_string();
        self
    }

    fn handle_math_block(mut self) -> Self {
        let re = regex::Regex::new(r"\$$(.*?)\$$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<MathBlock>r#$\"{}$\"#</MathBlock>r#\"", &caps[1])
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
fn trf(elm: &str) -> proc_macro2::TokenStream {
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

            output
                .replace("\n", " ")
                .parse::<proc_macro2::TokenStream>()
                .expect("Failed to parse Leptos view code")
        }
    }
}
