pub mod element_text;
pub mod transform;

use leptos::html::Tr;
use transform::Transformer;

fn main() {
    let transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider"],
        vec!["Paragraphs", "Example"],
        "Paragraph",
    );

    let html_code = transformer.transform(
        r#"
|> Section

    |> Paragraphs

        we

        |> okok 
            img
        
        |> than 
            img

        |> than 
            img

        |> than 
            img
            asd

            asdasd

        ...these forrmulas, commonly useful 
        
        in “applied” problems.

|> Example

    *Example 3.*

    "#
        .to_string(),
        0,
    );
    println!("{}", html_code);
}
