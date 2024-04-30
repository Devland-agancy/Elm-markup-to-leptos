pub mod element_text;
pub mod elm_json;
pub mod helpers;
pub mod transform;

use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::{fs::File, io::BufReader};
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

    let path = Path::new("src/elm.emu");
    // Open the file
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);

    let mut pre = transformer.pre_process_exercises(
        &r#"
|> hi 

    |> Solution

         hihi

    |> Solution

        hoho

    |> Solution

        hoho

    "#
        .to_string(),
    );

    pre = transformer.remove_empty_line_above(pre, vec!["ImageRight", "ImageLeft"]);
    pre = transformer.pre_process_solutions(pre);
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

    /*   let mut json_file = match File::create("src/json_output.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };
    let mut json = ElmJSON::new();
    let json_res = json.export_json(&contents);

    match file.write_all(json_res.as_bytes()) {
        Ok(_) => {
            println!("Json written to file successfully");
        }
        Err(error) => println!("Error writing to file: {}", error),
    } */
}
