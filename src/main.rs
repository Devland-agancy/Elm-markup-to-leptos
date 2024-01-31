pub mod element_text;
pub mod transform;

use transform::transform;

fn main() {
    let html_code = transform(
        r#"
|> Paragraph   

    can be positive or $negative or zero$( more on zero below),


    "#
        .to_string(),
    );
    println!("{}", html_code);
}
