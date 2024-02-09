pub mod element_text;
pub mod transform;

use leptos::html::Tr;
use transform::Transformer;

fn main() {
    let transformer: Transformer = Transformer::new(vec!["img", "SectionDivider"], "Paragraph");

    let html_code = transformer.transform(
        r#"
    |> Example

        *Example 1.*
        A line that passes through the points

        $$A = (-2, 5)$$
        
        and
        
        $$B = (4, 1)$$
        
        has slope
        
        $${1 - 5 \over 4 - (-2)} = {-4 \over 6} = - {2 \over 3}.$$
    "#
        .to_string(),
    );
    println!("{}", html_code);
}
