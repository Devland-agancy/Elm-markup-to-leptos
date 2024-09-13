pub mod counter;
pub mod datacell;
pub mod desugarer;
pub mod element_text;
pub mod emitter;
pub mod helpers;
pub mod parser;
pub mod parser_helpers;

use counter::counter_commands::CounterCommand;
use counter::counters::Counters;
use datacell::Datacell::*;
use desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use emitter::Emitter;
use parser::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

fn write_to_file(file_path: &str, contents: &str) {
    let mut json_file: File = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match json_file.write_all(contents.as_bytes()) {
        Ok(_) => (),
        Err(error) => println!("Error writing to {file_path}: {error}"),
    }
}

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

    //parsing
    let start = Instant::now();
    let mut parser = Parser::new();

    let parsed_json = parser.export_json(&contents, None, false);

    if let Err(err) = parsed_json {
        panic!("{}", err.to_string());
    }
    let parsed_json = parsed_json.unwrap();

    let last_item_id = parser.id;
    write_to_file("src/content_files/json_output.json", &parsed_json);
    println!("Time for parsing is: {:?}", start.elapsed());

    //desugering
    let start = Instant::now();
    let mut json_desugarer: Desugarer = Desugarer::new(parsed_json.as_str(), last_item_id);
    let article_types = &vec!["Chapter".to_string(), "Bootcamp".to_string()];
    json_desugarer = json_desugarer
        .pre_process_exercises()
        .add_increamental_attr(
            vec![("Solution", "solution_number"), ("Grid", "id")],
            article_types,
        )
        .auto_increamental_title("Exercise", "Exercise", article_types)
        .auto_increamental_title("Example", "Example", article_types)
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
        .add_indent()
        .add_attribute(vec!["Solution", "Example"], ("no_padding", "true"))
        .auto_convert_to_float(vec!["line", "padding_left"]);
    write_to_file(
        "src/content_files/des_json_output.json",
        &json_desugarer.json,
    );
    println!("Time for desugering is: {:?}", start.elapsed());

    let start = Instant::now();
    let mut desugarer_json_cell: DataCell = serde_json::from_str(&json_desugarer.json).unwrap();
    let mut counters = Counters::new();
    counters.get_counters_from_json(&desugarer_json_cell);
    let mut counter_command = CounterCommand::new(&mut counters, &json_desugarer.json);
    let json_counter_string = counter_command.run(&mut desugarer_json_cell);
    println!("Time for counter logic is: {:?}", start.elapsed());

    write_to_file("src/content_files/counter.json", &json_counter_string);

    // Emmitter
    let start = Instant::now();

    let json_value: DataCell = serde_json::from_str(&json_counter_string).unwrap();
    let mut emitter: Emitter = Emitter::new(vec![
        "img",
        "col",
        "SectionDivider",
        "InlineImage",
        "StarDivider",
        "br",
    ]);
    let leptos_code = emitter.emit_json(&json_value);

    let file_content = format!(
        r#"
    view! {{
        {}
    }}
    "#,
        leptos_code
    );

    write_to_file("src/content_files/output.rs", &file_content);
    let _ = Command::new("leptosfmt")
        .arg("src/content_files/output.rs")
        .output()
        .expect("Failed to execute command");

    println!("Time for emitting is: {:?}", start.elapsed());
}
