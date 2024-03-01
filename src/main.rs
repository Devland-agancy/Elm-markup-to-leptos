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

        on both sides by $x-x_0$, we find the fraction-less equation

        $$
        p(x-x_0) = y-y_0
        $$

        which is satisfied by the point $(x,y) = (x_0,y_0)$ as well as
        by every other point on the line.
        

    "#
        .to_string(),
        0,
    );
    println!("{}", html_code);
}
