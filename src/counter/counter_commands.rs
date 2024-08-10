use crate::parser_helpers::{Cell, CellType, DataCell, ElementCell};

use super::{
    counter_instance::CounterInstance, counter_types::CounterValueType, counters::Counters,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CommandType {
    #[default]
    INC,
    DEC,
    ASSIGN,
    INSERT,
}

impl CommandType {
    fn from_str(string: &str) -> Vec<Self> {
        match string {
            "::++" => vec![CommandType::INC, CommandType::INSERT],
            "::--" => vec![CommandType::DEC, CommandType::INSERT],
            "..++" => vec![CommandType::INC],
            "..--" => vec![CommandType::DEC],
            "::::" => vec![CommandType::INSERT],
            _ => vec![],
        }
    }

    fn to_tag_name(&self) -> String {
        match self {
            CommandType::INC => "CounterIncrement".to_string(),
            CommandType::DEC => "CounterDecrement".to_string(),
            CommandType::ASSIGN => "CounterAssign".to_string(),
            CommandType::INSERT => "CounterInsert".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CounterCommand {
    //commands: Vec<CommandType>,
}

pub struct LineSplits {
    pub content: String,
    pub is_command: bool,
}

impl CounterCommand {
    pub fn new() -> Self {
        Self {}
    }

    fn get_commands(&self, command_str: &str) -> Vec<CommandType> {
        let command_operation = &command_str[0..4]; // remove the first char if there one before the command as it's used to detect escape char

        CommandType::from_str(command_operation)
    }

    pub fn split_line(&self, line: &str) -> Vec<LineSplits> {
        let cmd_regex = r"(^|[^\\])(::|\.\.)(::|\+\+|--)\w+";

        // Create the regex object
        let re = Regex::new(cmd_regex).unwrap();

        // Find all matches and split the string accordingly
        let mut result = Vec::new();
        let mut start = 0;

        for mat in re.find_iter(line) {
            if mat.start() != start {
                result.push(LineSplits {
                    content: line[start..mat.start()].to_string(),
                    is_command: false,
                });
            }
            result.push(LineSplits {
                content: mat.as_str().to_string(),
                is_command: true,
            });
            start = mat.end();
        }

        // Add the remaining part of the string, if any
        if start < line.len() {
            result.push(LineSplits {
                content: line[start..].to_string(),
                is_command: false,
            });
        }
        result
    }

    pub fn generate_counter_elements(
        &self,
        command_str: &str,
        counters: &mut Counters,
        json_tree: &mut DataCell,
        latest_id: &mut u32,
        parent_id: u32,
    ) {
        // create regex to match this string ""::::CounterName"

        let command_str = if &command_str[0..2] != "::" || &command_str[0..2] != ".."
        // first char can be something else captured by regex for using (^|[^\\])
        {
            &command_str[1..]
        } else {
            command_str
        }; // remove the first char if there one before the command as it's used to detect escape char

        let detected_counter_name = &command_str[4..];

        let mut counter_names = Vec::new();
        for counter in counters.counters_list.iter() {
            counter_names.push(&counter.name)
        }
        if !counter_names.contains(&&detected_counter_name.to_string()) {
            panic!("Counter {} was used out of scope", detected_counter_name);
        }

        for command in self.get_commands(command_str) {
            let mut counter_element =
                ElementCell::init_cell(*latest_id, parent_id, &command.to_tag_name());
            *latest_id += 1;
            if let CellType::Element(el) = &mut counter_element.cell_type {
                el.push_attribute(&format!("counter_name {}", detected_counter_name));
                if command == CommandType::INC {
                    el.push_attribute(&format!("increment 1"));
                } else if command == CommandType::ASSIGN {
                    //el.push_attribute(&format!("value {}", 1));
                }
                el.is_counter = true;
                //el.push_attribute(&format!("silent {}", &command_str[0..2] == ".."));

                ElementCell::add_existing_cell(json_tree, parent_id, &counter_element);
            };
        }

        // for counter in counters.counters_list.iter_mut() {
        //     if command_str == format!("::++{}", counter.name)
        //         || command_str == format!("..++{}", counter.name)
        //     {
        //         counter.increment();
        //     }
        //     if command_str == format!("::--{}", counter.name)
        //         || command_str == format!("..--{}", counter.name)
        //     {
        //         counter.decrement();
        //     }

        //     if detected_counter_name == counter.name {
        //         // ::++ and ::-- are silent and do not change the line
        //         let replace_with = if &command_str[0..2] == "::" {
        //             caps[1].to_string() + counter.current_value.to_string().as_str()
        //         } else {
        //             caps[1].to_string()
        //         };
        //         new_line = insert_regex.replace(&new_line, &replace_with).to_string()
        //     }
        // }
    }
}

//create a test for replacecounter_in_line function
// #[test]
// fn test_replace_counter_in_line() {
//     let mut counters = Counters::new();
//     counters.add_counter(CounterInstance::new(
//         "TestArabicCounter",
//         "counter",
//         0,
//         None,
//     ));
//     counters.add_counter(CounterInstance::new(
//         "TestRomanCounter",
//         "roman_counter",
//         0,
//         None,
//     ));

//     //with text after
//     let test1 =
//         replace_counter_in_line("askljdklasj ::::TestArabicCounter qweqweqwe", &mut counters);
//     assert_eq!(test1, "askljdklasj 0 qweqweqwe");

//     //no text after
//     let test2 = replace_counter_in_line("askljdklasj ::::TestArabicCounter", &mut counters);
//     assert_eq!(test2, "askljdklasj 0");

//     // 2 same type counters in 1 line
//     let test3 = replace_counter_in_line(
//         "askljdklasj ::::TestArabicCounter hi ::::TestArabicCounter",
//         &mut counters,
//     );
//     assert_eq!(test3, "askljdklasj 0 hi 0");

//     // 2 different type counters in 1 line
//     let test4 = replace_counter_in_line(
//         "askljdklasj ::::TestArabicCounter hi ::::TestRomanCounter",
//         &mut counters,
//     );
//     assert_eq!(test4, "askljdklasj 0 hi 0");

//     // test increment
//     let test4 = replace_counter_in_line(
//         "askljdklasj ::++TestArabicCounter hi ::++TestRomanCounter",
//         &mut counters,
//     );
//     assert_eq!(test4, "askljdklasj 1 hi i");

//     // test double increment than normal
//     let test4 = replace_counter_in_line(
//         "inc ::++TestRomanCounter another inc ::++TestRomanCounter fix ::::TestRomanCounter",
//         &mut counters,
//     );
//     assert_eq!(test4, "inc ii another inc iii fix iii");

//     // test double deccrement than normal
//     let test4 = replace_counter_in_line(
//         "dec ::--TestRomanCounter another dec ::--TestRomanCounter fix ::::TestRomanCounter dec ar ::--TestArabicCounter fix ar ::::TestArabicCounter",
//         &mut counters,
//     );
//     assert_eq!(test4, "dec ii another dec i fix i dec ar 0 fix ar 0");

//     // test escaped
//     let test4 = replace_counter_in_line(r#"escaped \::--TestRomanCounter"#, &mut counters);
//     assert_eq!(test4, r"escaped \::--TestRomanCounter");
// }

// #[test]
// fn decrement_less_than_0() {
//     let mut counters = Counters::new();
//     counters.add_counter(CounterInstance::new(
//         "TestArabicCounter",
//         "counter",
//         0,
//         None,
//     ));
//     counters.add_counter(CounterInstance::new(
//         "TestRomanCounter",
//         "roman_counter",
//         0,
//         None,
//     ));

//     let test =
//         replace_counter_in_line("::--TestArabicCounter ::--TestArabicCounter", &mut counters);
//     assert_eq!(test, "- -");

//     let test2 =
//         replace_counter_in_line("::::TestRomanCounter ::--TestArabicCounter", &mut counters);
//     assert_eq!(test2, "0 -");
// }

// #[test]
// fn silent_operations() {
//     let mut counters = Counters::new();
//     counters.add_counter(CounterInstance::new(
//         "TestArabicCounter",
//         "counter",
//         0,
//         None,
//     ));

//     let test =
//         replace_counter_in_line("::++TestArabicCounter ..--TestArabicCounter", &mut counters);
//     assert_eq!(test, "1 ");

//     let test2 = replace_counter_in_line("::::TestArabicCounter", &mut counters);
//     assert_eq!(test2, "0");
// }

// #[test]
// fn default_value() {
//     let mut counters = Counters::new();
//     counters.add_counter(CounterInstance::new(
//         "TestArabicCounter",
//         "counter",
//         0,
//         Some("10"),
//     ));
//     counters.add_counter(CounterInstance::new(
//         "TestRomanCounter",
//         "roman_counter",
//         0,
//         Some("ii"),
//     ));

//     let test = replace_counter_in_line("::--TestArabicCounter", &mut counters);
//     assert_eq!(test, "9");

//     let test2 = replace_counter_in_line("::::TestRomanCounter ::--TestRomanCounter", &mut counters);
//     assert_eq!(test2, "ii i");
// }
