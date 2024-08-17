pub mod counter;
pub mod desugarer;
pub mod element_text;
pub mod emitter;
pub mod helpers;
pub mod parser;
pub mod parser_helpers;

use counter::counter_commands::CounterCommand;
use counter::counters::Counters;
use desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use emitter::Emitter;
use parser::Parser;
use parser_helpers::DataCell;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
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

    let mut counters = Counters::new();
    let mut json = Parser::new(&mut counters);

    let content_str = &contents;
    let json_tree = json.export_json(&contents, None, false);

    println!("counters_list {:?}", counters.counters_list);

    let mut counter_command = CounterCommand::new(&mut counters, &json_tree);
    let mut json: DataCell = serde_json::from_str(&json_tree).unwrap();

    let json_tree = counter_command.replace_counters(&mut json);

    let mut json_desugarer: Desugarer = Desugarer::new(json_tree.as_str(), json.id);
    json_desugarer = json_desugarer
        .pre_process_exercises()
        .add_increamental_attr(vec![("Solution", "solution_number"), ("Grid", "id")])
        .auto_increamental_title("Exercise", "Exercise")
        .auto_increamental_title("Example", "Example")
        .wrap_block_delimited("InnerParagraph")
        .wrap_children(
            vec!["Section", "Solution", "Example", "Exercise"],
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
                IgnoreOptions {
                    element: "del",
                    attach_to: AttachToEnum::BOTH,
                },
                IgnoreOptions {
                    element: "Space",
                    attach_to: AttachToEnum::BEFORE,
                },
                IgnoreOptions {
                    element: "CounterInsert",
                    attach_to: AttachToEnum::BOTH,
                },
            ]),
        )
        .wrap_children(
            vec!["Grid"],
            "Span",
            &Some(vec![
                IgnoreOptions {
                    element: "CounterInsert",
                    attach_to: AttachToEnum::BOTH,
                },
                IgnoreOptions {
                    element: "CounterIncrement",
                    attach_to: AttachToEnum::BOTH,
                },
            ]),
        )
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
                "Example", "Section", "tr", "Table", "Solution", "Exercise",
            ],
        })
        .add_attribute(vec!["Solution", "Example"], ("no_padding", "true"))
        .auto_convert_to_float(vec!["line"]);

    let json_value: DataCell = serde_json::from_str(&json_desugarer.json).unwrap();

    let mut emitter: Emitter = Emitter::new(
        &json_value,
        vec!["img", "SectionDivider", "InlineImage"],
        &mut counters,
    );
    let leptos_code = emitter.emit_json(&json_value);

    let mut file = match File::create("src/content/output.rs") {
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
            println!("Data written to output.rs successfully");
            let _ = Command::new("leptosfmt")
                .arg("src/content/output.rs")
                .output()
                .expect("Failed to execute command");
        }
        Err(error) => println!("Error writing to output.rs: {}", error),
    }

    let mut json_file = match File::create("src/content/json_output.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match json_file.write_all(json_tree.as_bytes()) {
        Ok(_) => {
            println!("Json written to json_output.json successfully");
        }
        Err(error) => println!("Error writing to json_output.json: {}", error),
    }

    let mut desagurer_json_file = match File::create("src/content/des_json_output.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match desagurer_json_file.write_all(json_desugarer.json.as_bytes()) {
        Ok(_) => {
            println!("Json written to des_json_output.json successfully");
        }
        Err(error) => println!("Error writing to des_json_output.json: {}", error),
    }
}
