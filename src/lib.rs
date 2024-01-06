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
struct DelimeterRules {
    symbol: &'static str,
    left_replacement: &'static str,
    right_replacement: &'static str,
    no_break: bool,
    keep_delimiter: bool,
}

const DELIMETERS: [DelimeterRules; 5] = [
    DelimeterRules {
        symbol: "*",
        left_replacement: "<Span bold=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "__",
        left_replacement: "<Paragragh Align::Center><Span italic=true>",
        right_replacement: "</Span></Paragraph>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "_",
        left_replacement: "<Span italic=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "$$",
        left_replacement: "<MathBlock>",
        right_replacement: "</MathBlock>",
        no_break: true,
        keep_delimiter: true,
    },
    DelimeterRules {
        symbol: "$",
        left_replacement: "<Math>",
        right_replacement: "</Math>",
        no_break: true,
        keep_delimiter: true,
    },
];

impl ContentLine {
    pub fn new(text: &str) -> ContentLine {
        ContentLine {
            text: text.to_string(),
        }
    }

    pub fn handle_delimeters(self) -> String {
        let symbols: Vec<&str> = DELIMETERS.iter().map(|d| d.symbol).collect();
        let mut i = 0;
        let mut j = 0;

        let mut output = String::new();

        let mut found_symbol: &str = "";

        while i < self.text.len() {
            while i < self.text.len()
                && ((i > 0 && self.get_char(i - 1) == "\\")
                    || !symbols.contains(&self.get_char(i).as_str()))
            {
                output.push_str(&self.get_char(i).as_str());
                i = i + 1;
                j = i;
            }

            if i < self.text.len() {
                let c = &self.get_char(i)[..];
                let next_char = &self.get_char(i + 1)[..];

                if next_char == c {
                    found_symbol = self.get_slice(i, i + 1).unwrap();

                    i = i + 2;
                    j = i;
                    while i < self.text.len()
                        && (self.get_char(i - 1) == "\\"
                            || (i < self.text.len() - 1
                                && found_symbol != self.get_slice(i, i + 1).unwrap()))
                    {
                        i = i + 1;
                    }
                    if i == self.text.len() - 1 {
                        // not found , we add one so that  i < self.text.len()
                        i = i + 1
                    }
                } else {
                    found_symbol = c;
                    i = i + 1;
                    j = i;
                    while i < self.text.len()
                        && (self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_char(i).as_str())
                    {
                        i = i + 1;
                    }
                }

                if i < self.text.len() {
                    let del = self.get_delimeter(&found_symbol);
                    let mut char_after_closing_del = "";
                    if i + 1 < self.text.len() && del.no_break {
                        char_after_closing_del = self.get_slice(i + 1, i + 1).unwrap();
                        if next_char == c && i + 2 < self.text.len() {
                            char_after_closing_del = self.get_slice(i + 2, i + 2).unwrap();
                        }
                    }
                    if char_after_closing_del != " " && char_after_closing_del != "" && del.no_break
                    {
                        output.push_str("\"#<span class=\"nobreak\">");
                    } else {
                        output.push_str("\"#");
                    }
                    output.push_str(del.left_replacement);
                    output.push_str("r#\"");
                    if del.keep_delimiter {
                        output.push_str(found_symbol);
                    }
                    i = j;
                    if next_char == c {
                        while self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_slice(i, i + 1).unwrap()
                        {
                            output.push_str(self.get_char(i).as_str());
                            i = i + 1
                        }
                        i = i + 2
                    } else {
                        while self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_char(i).as_str()
                        {
                            output.push_str(self.get_char(i).as_str());
                            i = i + 1
                        }
                        i = i + 1
                    }

                    if del.keep_delimiter {
                        output.push_str(found_symbol);
                    }
                    output.push_str("\"#");
                    output.push_str(del.right_replacement);
                    if del.no_break && char_after_closing_del != " " && char_after_closing_del != ""
                    {
                        output.push_str("</span>r#\"");
                    } else {
                        output.push_str("r#\"");
                    }
                } else {
                    // closing del not found , we push the symbol as normal text and continue
                    output.push_str(found_symbol);
                    i = j;
                }
            }
        }

        output
    }

    fn get_delimeter(&self, symbol: &str) -> &DelimeterRules {
        DELIMETERS.iter().find(|d| d.symbol == symbol).unwrap()
    }

    fn get_char(&self, i: usize) -> String {
        self.text
            .chars()
            .nth(i)
            .unwrap_or(" ".chars().nth(0).unwrap())
            .to_string()
    }

    fn get_slice(&self, start: usize, end: usize) -> Option<&str> {
        assert!(end >= start);
        let s = &self.text;
        let mut iter = s
            .char_indices()
            .map(|(pos, _)| pos)
            .chain(Some(s.len()))
            .skip(start)
            .peekable();
        let start_pos = *iter.peek()?;
        for _ in start..end {
            iter.next();
        }
        Some(&s[start_pos..*iter.peek()?])
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

    output
        .replace("\n", " ")
        .parse::<proc_macro2::TokenStream>()
        .expect("Failed to parse Leptos view code")
}
