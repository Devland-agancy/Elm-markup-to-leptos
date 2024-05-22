pub mod element_text;
pub mod elm_json;
pub mod elm_json_helpers;
pub mod emitter;
pub mod helpers;
pub mod parser;

use elm_json::ElmJSON;
use emitter::Emitter;
use parser::{AutoWrapper, Parser};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::{env, fs::File, io::BufReader};
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
        Ok(_) => println!("File contents:\n{}", contents),
    }

    let mut emitter: Emitter = Emitter::new(&contents);
    let mut parser: Parser = Parser::new(
        vec!["img", "SectionDivider", "InlineImage"],
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

    emitter = emitter
        .pre_process_exercises()
        .remove_empty_line_above(
            vec!["ImageRight", "ImageLeft"],
            Some(("attached", "false")),
            &mut parser,
        )
        .pre_process_solutions()
        .auto_increamental_title("Example", "Example", None, None)
        .auto_increamental_title(
            "Exercise",
            "Exercise",
            Some("ExerciseQuestion"),
            Some("Solution"),
        );

    let leptos_code = parser.transform(emitter.elm, 0);
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
    let mut json = ElmJSON::new();
    let json_tree = json.export_json(&contents.to_string(), None, false);

    match json_file.write_all(json_tree.as_bytes()) {
        Ok(_) => {
            println!("Json written to file successfully");
        }
        Err(error) => println!("Error writing to file: {}", error),
    }
}
