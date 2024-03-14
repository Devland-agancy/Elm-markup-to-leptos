pub mod element_text;
pub mod transform;
use leptos::html::Tr;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use transform::Transformer;

fn main() {
    let mut transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider"],
        vec!["Paragraphs", "Example", "Section", "Solution"],
        vec!["Example"],
        "Paragraph",
        vec![
            "Image",
            "DisplayImage",
            "Pause",
            "InlineImage",
            "MathBlock",
            "Table",
        ],
        vec![
            "Paragraphs",
            "Paragraph",
            "Example",
            "Section",
            "tr",
            "Table",
            "Solution",
        ],
    );

    let pre = transformer.pre_process_exercises(
        r#"
|> Section

    ok 

    $$ Unfortunately, the equation $$
    |> ImageRight
        ok

|> Exercise
    hi

|> Exercise
    hi

|> ok
    "#
        .to_string(),
    );
    let leptos_code = transformer.transform(pre, 0);
    let mut file = match File::create("src/output.rs") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };
    let file_content = format!(
        r#"
    view! {{
        {}
    }}
    "#,
        leptos_code
    );
    match file.write_all(file_content.as_bytes()) {
        Ok(_) => {
            println!(
            "Data written to file successfully , use leptosfmt to format leptos for better view"
        );
            let _ = Command::new("leptosfmt")
                .arg("src/output.rs")
                .output()
                .expect("Failed to execute command");
        }
        Err(error) => println!("Error writing to file: {}", error),
    }
}
