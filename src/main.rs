pub mod element_text;
pub mod transform;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use transform::{AutoWrapper, Transformer};

fn main() {
    let mut transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider"],
        vec![
            AutoWrapper {
                tags: vec!["Paragraphs", "Example", "Section", "Solution"],
                wrap_children_with: "Paragraph",
                enable_manual_wrap: true,
            },
            AutoWrapper {
                tags: vec!["Grid"],
                wrap_children_with: "Span",
                enable_manual_wrap: true,
            },
            AutoWrapper {
                tags: vec!["List"],
                wrap_children_with: "Item",
                enable_manual_wrap: true,
            },
        ],
        vec!["Example"],
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
        vec!["Grid", "List"],
    );

    let mut pre = transformer.pre_process_exercises(
        r#"|> Section
    classes="animate-appear-slow"


    for every real number $x$, where $|x|$ 
    denotes the absolute value of $x$.

    (Nb: If ever you want to indicate both solutions of the
    equation $x^2 = 4$ you can always use the notation.
    “$\pm \sqrt{4}$” 
    This is what happens, for example, in the 
    maybe-well-known formula

    "#
        .to_string(),
    );

    pre = transformer.auto_increamental_title(pre, "Example", "Example", None, None);
    pre = transformer.auto_increamental_title(
        pre,
        "Exercise",
        "Exercise",
        Some("ExerciseQuestion"),
        Some("Solution"),
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
