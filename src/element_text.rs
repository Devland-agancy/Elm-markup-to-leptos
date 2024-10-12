use std::cell::Cell;

use crate::datacell::{BlockChildType::*, CellTrait::Cell, Datacell::*, ElementCell::*};

pub struct ElementText {
    pub text: String,
    rules: Vec<DelimeterRules>,
}

#[derive(Debug)]
struct DelimeterRules {
    symbol: &'static str,
    end_symbol: &'static str,
    element_name: &'static str,
    props: Vec<Prop>,
    no_break: bool,
    keep_delimiter: bool,
    ignore_nested_delimeters: bool,
    keep_escaped_char_when_closed: bool,
    ignore_when_before: Vec<char>,
    ignore_when_after: Vec<char>,
}

impl Default for DelimeterRules {
    fn default() -> Self {
        Self {
            symbol: "",
            end_symbol: "",
            element_name: "",
            props: Vec::new(),
            no_break: false,
            keep_delimiter: false,
            ignore_nested_delimeters: false,
            ignore_when_before: Vec::new(),
            ignore_when_after: Vec::new(),
            keep_escaped_char_when_closed: false,
        }
    }
}

impl ElementText {
    pub fn new(text: &str) -> ElementText {
        ElementText {
            text: text.to_string(),
            rules: vec![
                DelimeterRules {
                    symbol: "*",
                    end_symbol: "*",
                    element_name: "Span",
                    props: vec![Prop {
                        key: "bold".to_string(),
                        value: "true".to_string(),
                    }],
                    ignore_when_after: vec!['(', '[', '{', '*', ' '],
                    ignore_when_before: vec![')', ']', '}', '*', ' '],
                    ..Default::default()
                },
                DelimeterRules {
                    symbol: "__",
                    end_symbol: "__",
                    element_name: "Span",
                    props: vec![
                        Prop {
                            key: "italic".to_string(),
                            value: "true".to_string(),
                        },
                        Prop {
                            key: "align".to_string(),
                            value: "center".to_string(),
                        },
                    ],
                    ..Default::default()
                },
                DelimeterRules {
                    symbol: "_|",
                    end_symbol: "|_",
                    element_name: "Span",
                    props: vec![Prop {
                        key: "align".to_string(),
                        value: "center".to_string(),
                    }],
                    ..Default::default()
                },
                DelimeterRules {
                    symbol: "_",
                    end_symbol: "_",
                    element_name: "Span",
                    props: vec![Prop {
                        key: "italic".to_string(),
                        value: "true".to_string(),
                    }],
                    ignore_when_after: vec!['(', '[', '{', ' '],
                    ignore_when_before: vec![')', ']', '}', ' '],
                    ..Default::default()
                },
                DelimeterRules {
                    symbol: "$$",
                    end_symbol: "$$",
                    element_name: "Mathblock",
                    no_break: true,
                    keep_delimiter: true,
                    ignore_nested_delimeters: true,
                    ..Default::default()
                },
                DelimeterRules {
                    symbol: "$",
                    end_symbol: "$",
                    element_name: "Math",
                    no_break: true,
                    keep_delimiter: true,
                    ignore_nested_delimeters: true,
                    keep_escaped_char_when_closed: true, // Ex: $ ex \$ $ --> <Math>$ ex \$ $</Math> instead of <Math>$ ex $ $</Math>
                    ..Default::default()
                },
            ],
        }
    }

    // pub fn handle_delimeters(&self) -> String {
    //     let mut i = 0;
    //     let mut output = String::new();

    //     while i <= self.text.len() {
    //         let (del, skips, text) = &self.find_next_delimeter(i, false);

    //         output.push_str(text);

    //         if del.is_none() {
    //             break;
    //         }

    //         i = *skips;

    //         if i <= self.text.len() {
    //             let (found, closing_index, del_content) =
    //                 &self.find_closing_delimeter(i, &del.unwrap(), false);

    //             if !found {
    //                 // closing del not found , we push the symbol as normal text and continue

    //                 output.push_str(&del.unwrap().symbol);
    //                 let nested_content = ElementText::new(&del_content).handle_delimeters();

    //                 output.push_str(nested_content.as_str());
    //                 i = *closing_index + 1;
    //                 continue;
    //             }

    //             let nested_content = if !del.unwrap().ignore_nested_delimeters {
    //                 ElementText::new(&del_content).handle_delimeters()
    //             } else {
    //                 del_content.to_string()
    //             };

    //             if i <= self.text.len() {
    //                 i = closing_index + del.unwrap().end_symbol.len();
    //                 let mut char_after_closing_del = "";
    //                 if i <= self.text.len() && del.unwrap().no_break {
    //                     char_after_closing_del = &self.get_slice(i, i + 1).unwrap_or("");
    //                 }

    //                 if char_after_closing_del != " "
    //                     && char_after_closing_del != ""
    //                     && del.unwrap().no_break
    //                 {
    //                     // remove prev chars until we hit a space
    //                     let mut removed = String::new();
    //                     while output.len() > 0 && output.chars().last().unwrap() != ' ' {
    //                         // replace first char of removed
    //                         removed = format!("{}{}", output.pop().unwrap(), removed);
    //                     }
    //                     output.push_str("\"#<span class=\"nobreak\">r#\"");
    //                     output.push_str(removed.as_str());
    //                     output.push_str("\"#");
    //                 } else {
    //                     output.push_str("\"#");
    //                 }
    //                 output.push_str(del.unwrap().left_replacement);
    //                 output.push_str("r#\"");

    //                 if del.unwrap().keep_delimiter {
    //                     output.push_str(&del.unwrap().symbol);
    //                 }

    //                 output.push_str(&nested_content);

    //                 if del.unwrap().keep_delimiter {
    //                     output.push_str(&del.unwrap().end_symbol);
    //                 }
    //                 output.push_str("\"#");
    //                 output.push_str(del.unwrap().right_replacement);
    //                 if del.unwrap().no_break
    //                     && char_after_closing_del != " "
    //                     && char_after_closing_del != ""
    //                 {
    //                     output.push_str("r#\"");
    //                     let mut string = "".to_string();
    //                     while i < self.text.len()
    //                         && self.get_char(i) != " "
    //                         && self.get_char(i) != ""
    //                     {
    //                         string.push_str(self.get_char(i).as_str());
    //                         i += 1;
    //                     }
    //                     let handled_string = self::ElementText::new(&string).handle_delimeters();
    //                     i += 1;
    //                     output.push_str(&handled_string);
    //                     output.push_str("\"#</span>r#\"");
    //                 } else {
    //                     output.push_str("r#\"");
    //                     i += 1;
    //                 }
    //             }
    //         }
    //     }
    //     output
    // }

    pub fn split_text(&self) -> Vec<CellType> {
        let mut i = 0;
        let mut output = Vec::<CellType>::new();
        while i <= self.text.len() {
            let (del, skips, text) = &self.find_next_delimeter(i, true);

            let mut text_cell = text.clone();

            if !del.is_some() {
                break;
            }
            let del = del.unwrap();

            i = *skips;

            if i <= self.text.len() {
                let (found, closing_index, del_content) =
                    &self.find_closing_delimeter(i, &del, true);

                if !found {
                    // closing del not found , we push the symbol as normal text and continue

                    text_cell.push_str(&del.symbol);
                    // let nested_content = ElementText::new(&del_content).handle_delimeters();
                    text_cell.push_str(&del_content);

                    output.push(CellType::create_block_from_text(&text_cell));
                    i = *closing_index + 1;

                    continue;
                }
                output.push(CellType::create_block_from_text(&text_cell));

                if i <= self.text.len() {
                    i = closing_index + del.end_symbol.len();

                    let del_content_block = CellType::create_block_from_text(&del_content)
                        .as_block()
                        .unwrap();

                    let element = ElementCell {
                        name: del.element_name.to_string(),
                        props: del.props.clone(),
                        children: vec![],
                    };

                    // output.push(BlockChildType::Delimited(DelimitedCell {
                    //     terminal: del_content.to_owned(),
                    //     open_delimeter: del.symbol.to_string(),
                    //     close_delimeter: del.end_symbol.to_string(),
                    //     display_type: if del.symbol.len() > 1 {
                    //         DelimitedDisplayType::BLOCK
                    //     } else {
                    //         DelimitedDisplayType::INLINE
                    //     },
                    //     wrapped_with: None,
                    // }));

                    i += 1;
                }
            }
        }
        output
    }

    pub fn remove_escapes(&mut self) -> &Self {
        let mut output = String::new();
        let mut i = 0;
        let symbols = self.get_all_symbols();
        let text = &self.text;

        // remove escape char if it's before a delimiter symbol
        while i + 1 < text.len() {
            if self.get_char(i) != "\\" {
                output.push_str(&self.get_char(i));
                i += 1;
                continue;
            }

            let mut delimiter_escaped = false;
            symbols.iter().any(|s| {
                if self.get_char(i + s.len()) == *s {
                    output.push_str(s);
                    i += s.len() + 1;
                    delimiter_escaped = true;
                    return true;
                }
                false
            });
            if !delimiter_escaped {
                output.push_str(&self.get_char(i));
                i += 1
            }
        }
        self.text = output;
        self
    }

    fn find_next_delimeter(
        &self,
        mut i: usize,
        keep_escape_char: bool,
    ) -> (Option<&DelimeterRules>, usize, String) {
        let mut found_symbol = "";
        let mut del: Option<&DelimeterRules> = None;
        let mut text = "".to_string();
        let symbols: Vec<&str> = self.rules.iter().map(|d| d.symbol).collect();
        let mut has_multi_char = false;

        while i <= self.text.len() {
            let _del = self.rules.iter().find(|d| {
                if i.checked_sub(d.symbol.len()).is_some() {
                    d.symbol
                        == &self
                            .text
                            .chars()
                            .take(i)
                            .skip(i - d.symbol.len())
                            .collect::<String>()
                        && self.text.chars().nth(i - d.symbol.len() + 1).is_some()
                        && !d
                            .ignore_when_before
                            .contains(&self.text.chars().nth(i - d.symbol.len() + 1).unwrap())
                } else {
                    false
                }
            });

            if _del.is_some() {
                if symbols.contains(
                    // in case founded delimiter is same as another delimeter first char e.g $ and $$
                    &self
                        .text
                        .chars()
                        .take(i + 1)
                        .skip(i - _del.unwrap().symbol.len())
                        .collect::<String>()
                        .as_str(),
                ) {
                    i += 1;
                    has_multi_char = true;
                    continue;
                }
                del = _del;
                found_symbol = _del.unwrap().symbol;
                if !has_multi_char {}
                if self.is_escaped(i - found_symbol.len()) {
                    let mut nth = text.chars().nth(i);
                    while nth.is_some() && nth.unwrap() != '\\' {
                        text.pop();
                        nth = text.chars().nth(i);
                    }

                    if !keep_escape_char {
                        // pop the "\"
                        text.pop();
                    }

                    text.push_str(found_symbol); // push again without "\"
                    i += 1;

                    continue;
                }
                break;
            }

            if i.checked_sub(1).is_some() && !(i == 1 && &self.get_char(i - 1) == " ") {
                text.push_str(&self.get_char(i - 1));
            }
            i += 1;
        }

        (del, i, text)
    }

    fn find_closing_delimeter(
        &self,
        mut i: usize,
        found_del: &DelimeterRules,
        keep_escape_char: bool,
    ) -> (bool, usize, String) {
        let end_symbol = found_del.end_symbol;
        let mut found = false;
        let mut del_content = "".to_string();
        let symbols: Vec<&str> = self.rules.iter().map(|d| d.symbol).collect();

        while i < self.text.len() {
            let is_found = end_symbol
                == &self
                    .text
                    .chars()
                    .take(i + end_symbol.len())
                    .skip(i)
                    .collect::<String>()
                && !found_del
                    .ignore_when_after
                    .contains(&self.text.chars().nth(i - 1).unwrap());

            if !is_found {
                if (end_symbol
                    != &self
                        .text
                        .chars()
                        .take(i + end_symbol.len())
                        .skip(i)
                        .collect::<String>()
                    && !self.escape_before_symbol(i, &end_symbol))
                    || found_del
                        .ignore_when_after
                        .contains(&self.text.chars().nth(i - 1).unwrap())
                {
                    del_content.push_str(&self.get_char(i));
                }
                i += 1;

                continue;
            }
            let next_char = &self.text.chars().take(i + 2).skip(i).collect::<String>();
            if symbols.contains(
                // in case founded delimiter is same as another delimeter first char e.g $ and $$
                // this is solution for this case $ xx $$ , we don't want xx $ to be considered as text inside $ $ ,
                &next_char.as_str(),
            ) && next_char != &end_symbol
            {
                i += 2;
                del_content.push_str(next_char);

                continue;
            }
            found = true;
            if self.is_escaped(i) {
                if keep_escape_char || found_del.keep_escaped_char_when_closed {
                    del_content.push_str("\\");
                }
                del_content.push_str(end_symbol);
                found = false;
                i += end_symbol.len();

                continue;
            }
            break;
        }

        (found, i, del_content)
    }

    fn get_char(&self, i: usize) -> String {
        if let Some(_char) = self.text.chars().nth(i) {
            _char.to_string()
        } else {
            "".to_string()
        }
    }

    fn is_escaped(&self, i: usize) -> bool {
        i > 0 && self.get_char(i - 1) == "\\"
    }

    fn get_all_symbols(&self) -> Vec<String> {
        let mut symbols: Vec<String> = self.rules.iter().map(|d| d.symbol.to_string()).collect();
        let end_symbols: Vec<String> = self
            .rules
            .iter()
            .map(|d| d.end_symbol.to_string())
            .collect();
        for end_s in end_symbols {
            if !symbols.contains(&end_s) {
                symbols.push(end_s)
            }
        }
        symbols
    }

    fn escape_before_symbol(&self, i: usize, symbol: &str) -> bool {
        i > 0
            && self
                .get_slice(i + 1, i + 1 + symbol.len())
                .is_some_and(|s| self.get_all_symbols().contains(&s.to_string()))
            && self.get_char(i) == "\\"
    }

    fn get_slice(&self, start: usize, end: usize) -> Option<&str> {
        assert!(end >= start);
        let s = &self.text;

        let mut iter = s
            .char_indices()
            .map(|(pos, _)| pos)
            .chain(Some(s.len()))
            .skip(start)
            .peekable();
        let start_pos = *iter.peek()?;
        for _ in start..end {
            iter.next();
        }

        Some(&s[start_pos..*iter.peek()?])
    }
}
