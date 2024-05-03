pub mod element_text;
pub mod elm_json;
pub mod emitter;
pub mod helpers;
pub mod parser;

use emitter::Emitter;
use parser::{AutoWrapper, Parser};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::{fs::File, io::BufReader};

fn main() {
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

    let mut emitter: Emitter = Emitter::new(
        r#"
|> Section

    $
    $$
    (v \ra v^2)(10) = 100
    $$
    $$
    (z \ra z^3)(10) + (t \ra t^2)(5) = 1025.
    $$

    (Etc.) (Indeed, to emphasize again, the variable 
    denoting the input does not matter: it is just a 
    placeholder, and you obtain the same output, and 
    the same <i>function</i>, no matter what symbol 
    you choose.*) (*As long as you don't collide with 
    other existing variable names.)

    is a “rule” for transforming inputs (usually
    numbers) into outputs (usually numbers as well).
    One can think of a function as a box with an
    “input tube” and an “output tube”:

    okhello

    hellos

    "#
        .to_string(),
    );
    let mut parser: Parser = Parser::new(
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

    emitter = emitter
        .pre_process_exercises()
        .remove_empty_line_above(
            vec!["ImageRight", "ImageLeft"],
            Some(("attached", "false")),
            parser.track_line_delta,
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
