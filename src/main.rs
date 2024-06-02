pub mod desugarer;
pub mod element_text;
pub mod emitter;
pub mod helpers;
pub mod parser;
pub mod parser_helpers;

use desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use emitter::{AutoWrapper, Emitter};
use parser::Parser;
use parser_helpers::DataCell;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::{fs::File, io::BufReader};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <file path>");
        return;
    }

    let path = Path::new(&args[1]);
    // Open the file
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    let mut json = Parser::new();
    let json_tree = json.export_json(&contents.to_string(), None, false);

    let mut json_desugarer: Desugarer = Desugarer::new(json_tree.as_str(), json.id);
    let mut desugarer: Desugarer = Desugarer::new(&contents.as_str(), json.id);

    let mut emitter: Emitter = Emitter::new(
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

    json_desugarer = json_desugarer
        .pre_process_exercises()
        .pre_process_solutions()
        .wrap_children(
            vec!["Section", "Solution", "Example"],
            "Paragraph",
            &Some(vec![
                IgnoreOptions {
                    element: "InlineImage",
                    attach_to: AttachToEnum::BOTH,
                },
                IgnoreOptions {
                    element: "ImageRight",
                    attach_to: AttachToEnum::BEFORE,
                },
                IgnoreOptions {
                    element: "ImageLeft",
                    attach_to: AttachToEnum::BEFORE,
                },
            ]),
        )
        .wrap_children(vec!["Grid"], "Span", &None)
        .wrap_children(vec!["List"], "Item", &None)
        .add_indent(&ParagraphIndentOptions {
            tags_before_non_indents: vec![
                "Image",
                "DisplayImage",
                "Pause",
                "InlineImage",
                "MathBlock",
                "Table",
            ],
            tags_with_non_indent_first_child: vec![
                "Paragraphs",
                "Paragraph",
                "Example",
                "Section",
                "tr",
                "Table",
                "Solution",
            ],
        });

    /* desugarer = desugarer
    .pre_process_exercises()
    .remove_empty_line_above(
        vec!["ImageRight", "ImageLeft"],
        Some(("attached", "false")),
        &mut emitter,
    )
    .pre_process_solutions()
    .auto_increamental_title("Example", "Example", None, None)
    .auto_increamental_title(
        "Exercise",
        "Exercise",
        Some("ExerciseQuestion"),
        Some("Solution"),
    ); */

    //let leptos_code = emitter.transform(desugarer.json, 0);
    let json_value: DataCell = serde_json::from_str(&json_desugarer.json).unwrap();
    let leptos_code = emitter.emit_json(&json_value);

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

    let mut json_file = match File::create("src/json_output.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match json_file.write_all(json_tree.as_bytes()) {
        Ok(_) => {
            println!("Json written to file successfully");
        }
        Err(error) => println!("Error writing to file: {}", error),
    }

    let mut desagurer_json_file = match File::create("src/des_json_output.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    let desagurer_json_tree = serde_json::to_string_pretty(&json_desugarer.json).unwrap();
    match desagurer_json_file.write_all(json_desugarer.json.as_bytes()) {
        Ok(_) => {
            println!("Json written to file successfully");
        }
        Err(error) => println!("Error writing to file: {}", error),
    }
}
