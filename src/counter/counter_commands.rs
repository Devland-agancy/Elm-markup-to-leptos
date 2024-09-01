use crate::datacell::{BlockChildType::*, Datacell::*};

use super::counters::Counters;
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
        let mut sub_string = string;
        if string.starts_with("<<") {
            sub_string = &string[2..];
        }

        let mut res = match sub_string {
            "::++" => vec![CommandType::INC, CommandType::INSERT],
            "::--" => vec![CommandType::DEC, CommandType::INSERT],
            "..++" => vec![CommandType::INC],
            "..--" => vec![CommandType::DEC],
            "::::" => vec![CommandType::INSERT],
            _ => vec![],
        };
        if string.starts_with("<<") {
            res.push(CommandType::ASSIGN); // align should be the last
        }
        res
    }

    fn to_tag_name(&self) -> String {
        match self {
            CommandType::INC => "CounterIncrement".to_string(),
            CommandType::DEC => "CounterDecrement".to_string(),
            CommandType::ASSIGN => "CounterAssign".to_string(),
            CommandType::INSERT => "CounterInsert".to_string(),
        }
    }

    pub fn from_tag_name(tag_name: &str) -> Option<Self> {
        match tag_name {
            "CounterIncrement" => Some(CommandType::INC),
            "CounterDecrement" => Some(CommandType::DEC),
            "CounterAssign" => Some(CommandType::ASSIGN),
            "CounterInsert" => Some(CommandType::INSERT),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CounterCommand<'a> {
    //commands: Vec<CommandType>,
    pub counters: &'a mut Counters,
    pub json_tree: String,
}

#[derive(Debug)]
pub struct LineSplits {
    pub content: String,
    pub is_command: bool,
}

impl<'a> CounterCommand<'a> {
    pub fn new(counters: &'a mut Counters, json_tree: &'a str) -> Self {
        Self {
            counters,
            json_tree: json_tree.to_string(),
        }
    }

    fn get_commands(&self, command_str: &str) -> Vec<CommandType> {
        let command_split = command_str.split("<<").collect::<Vec<&str>>();

        if command_split.len() == 1 {
            CommandType::from_str(&command_split[0][0..4])
        } else {
            CommandType::from_str(&("<<".to_string() + &command_split[1][0..4]))
        }
    }

    fn get_command_counter_name(&self, command_str: &str) -> String {
        let command_split = command_str.split("<<").collect::<Vec<&str>>();

        command_split.last().unwrap()[4..].to_string()
    }
    fn get_handle_name(&self, command_str: &str) -> Option<String> {
        let command_split = command_str.split("<<").collect::<Vec<&str>>();

        if command_split.len() == 1 {
            None
        } else {
            Some(command_split[0].to_string())
        }
    }

    fn split_line(&self, line: &str, handle_insert_command: bool) -> Vec<LineSplits> {
        let cmd_regex = if handle_insert_command {
            r"(^|[^\\])(>>)\w+"
        } else {
            r"(^|[^\\])(\w+<<)?(::|\.\.)(::|\+\+|--)\w+"
        };

        // Create the regex object
        let re = Regex::new(cmd_regex).unwrap();

        // Find all matches and split the string accordingly
        let mut result = Vec::new();
        let mut start = 0;

        for mat in re.find_iter(line) {
            let match_first_char = &line[mat.start()..mat.start() + 1];

            if mat.start() != start {
                //if first char of found regex is not a space, add the previous part of the string
                let mut string = String::from(&line[start..mat.start()]);
                string.push_str(match_first_char);
                result.push(LineSplits {
                    content: string,
                    is_command: false,
                });
            }

            let mut string = String::new();
            if mat.start() > 0 {
                string.push_str(&mat.as_str()[1..]);
            } else {
                string.push_str(&mat.as_str());
            }

            result.push(LineSplits {
                content: string,
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

    pub fn run(&mut self, json: &mut DataCell) -> String {
        if self.counters.counters_list.is_empty() {
            return self.json_tree.clone();
        }
        let json_str = self.replace_counters(json, false);
        let mut json: DataCell = serde_json::from_str(&json_str).unwrap();
        let json_str = self.replace_counters(&mut json, true);

        json_str
    }

    pub fn replace_counters(&mut self, cell: &mut DataCell, handle_insert_command: bool) -> String {
        let cloned_cell = cell.clone();

        match &mut cell.cell_type {
            CellType::Block(block) => {
                block.children.iter_mut().for_each(|child| {
                    self.handle_block_child(child, &cloned_cell, handle_insert_command)
                });
            }
            CellType::Element(el) => el.children.iter_mut().for_each(|child| {
                self.replace_counters(child, handle_insert_command);
            }),
            CellType::Root(el) => el.children.iter_mut().for_each(|child| {
                self.replace_counters(child, handle_insert_command);
            }),
            _ => (),
        }

        serde_json::to_string(&cell).unwrap()
    }

    fn handle_block_child(
        &mut self,
        block_child: &mut BlockChildType,
        cell: &DataCell,
        handle_insert_command: bool,
    ) {
        match block_child {
            BlockChildType::Text(text) => {
                text.content = if handle_insert_command {
                    self.insert_handle_value(&text.content)
                } else {
                    self.replace_counter_value(&text.content, cell)
                }
            }
            BlockChildType::Delimited(d) => {
                d.terminal = if handle_insert_command {
                    self.insert_handle_value(&d.terminal)
                } else {
                    self.replace_counter_value(&d.terminal, cell)
                };
            }
        };
    }

    fn replace_counter_value(&mut self, text: &str, cell: &DataCell) -> String {
        let splits = self.split_line(&text, false);
        let mut res = String::new();

        for split in splits {
            if split.is_command {
                let counter_name = &self.get_command_counter_name(split.content.as_str());
                self.counters.check_scope(
                    &cell,
                    counter_name,
                    &serde_json::from_str(&self.json_tree).unwrap(),
                );

                let commands = self.get_commands(split.content.as_str());

                let handle_name = self.get_handle_name(split.content.as_str());

                for command in commands {
                    let execution = self.counters.execute(command, counter_name, &handle_name);
                    if execution.is_some() {
                        if &split.content.as_str()[0..1] == " " {
                            res.push_str(" ");
                        }
                        res.push_str(&execution.unwrap());
                    }
                }
            } else {
                res.push_str(&split.content);
            }
        }

        res
    }

    fn insert_handle_value(&mut self, text: &str) -> String {
        let splits = self.split_line(&text, true);

        let mut res = String::new();
        for split in splits {
            if split.is_command {
                let handle = self
                    .counters
                    .handles_list
                    .iter()
                    .find(|h| h.name == &split.content[2..]);
                if handle.is_some() {
                    res.push_str(&handle.unwrap().value);
                };
            } else {
                res.push_str(&split.content);
            }
        }

        res
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
