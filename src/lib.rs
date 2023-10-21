
#[macro_use]
extern crate proc_macro;
extern crate nom;

use proc_macro::TokenStream;
use proc_macro2::{ Ident};
use quote::quote;
use syn::{parse_macro_input, LitStr};


use nom::{
    IResult,
    character::complete::{char, multispace0, none_of},
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
};
use regex::Regex;

struct Input{
    cx: Ident,
    elm: LitStr
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cx: syn::Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let elm: LitStr = input.parse()?;
        Ok(Input {
            cx,
            elm,
        })
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


    let leptos_code = transform_html_to_leptos_view(&elm.value());
    let output = quote! {
        view! {
            cx, #leptos_code
        }
    };
    println!("**********************************{}******************************************", output);
    output.into()
}


fn transform_html_to_leptos_view(elm: &str) -> proc_macro2::TokenStream {
    let mut html = String::new();
    let mut inside_code_block = false;

    for line in elm.lines() {
        if let Some(tag) = line.strip_prefix("|> ") {
            if !inside_code_block {
                html.push_str(&format!("<{}>", tag));
                inside_code_block = true;
            } else {
                html.push_str(&format!("</{}>\n", tag));
                inside_code_block = false;
            }
        } else if inside_code_block {
            html.push_str(line);
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html.parse::<proc_macro2::TokenStream >().expect("Failed to parse Leptos view code")

}


fn parse_attribute(input: &str) -> IResult<&str, (char, Vec<char>)> {
    let attribute_parser = delimited(multispace0, none_of(" \n="), multispace0);
    let mut key = terminated(attribute_parser, char('='));
    let mut value = opt(delimited(char('"'), many0(none_of("\"")), char('"')));
    let (input, key) = key(input)?;
    let (input, value) = value(input)?;
    Ok((input, (key, value.unwrap_or_default())))
}

fn parse_element(input: &str) -> IResult<&str, String> {
    let mut element_start = terminated(char('|') , multispace0);
    let (input, _) = element_start(input)?;
    let (input, tag) = terminated(none_of(" \n"), multispace0)(input)?;
    let attributes = many0(parse_attribute);
    let (input, _attributes) = opt(attributes)(input)?;
    let (input, content) = opt(delimited(char('{'), many0(none_of("}")), char('}')))(input)?;
    let content = content.unwrap_or_default();

    let mut result = format!("<{}{:?}>", tag, content);
    if let Some(attributes) = _attributes {
        for (key, value) in attributes {
            result = format!("{} {}=\"{:?}\"", result, key, value);
        }
    }
    Ok((input, result))
}


