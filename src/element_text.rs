use crate::parser_helpers::{BlockCell, BlockChild, BlockChildType, DelimitedCell, TextCell};

pub struct ElementText {
    pub text: String,
}

#[derive(Debug)]
struct DelimeterRules {
    symbol: &'static str,
    end_symbol: &'static str,
    left_replacement: &'static str,
    right_replacement: &'static str,
    no_break: bool,
    keep_delimiter: bool,
}

const DELIMETERS: [DelimeterRules; 6] = [
    DelimeterRules {
        symbol: "*",
        end_symbol: "*",
        left_replacement: "<Span bold=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "__",
        end_symbol: "__",
        left_replacement: "<Span italic=true align=Align::Center>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "_|",
        end_symbol: "|_",
        left_replacement: "<Span align=Align::Center>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "_",
        end_symbol: "_",
        left_replacement: "<Span italic=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "$$",
        end_symbol: "$$",
        left_replacement: "<MathBlock>",
        right_replacement: "</MathBlock>",
        no_break: true,
        keep_delimiter: true,
    },
    DelimeterRules {
        symbol: "$",
        end_symbol: "$",
        left_replacement: "<Math>",
        right_replacement: "</Math>",
        no_break: true,
        keep_delimiter: true,
    },
];

impl ElementText {
    pub fn new(text: &str) -> ElementText {
        ElementText {
            text: text.to_string(),
        }
    }

    pub fn handle_delimeters(self) -> String {
        let mut i = 0;
        let mut j = 0;
        let mut output = String::new();

        while i <= self.text.len() {
            let (del, skips, text) = &self.find_next_delimeter(i);

            if text == " " {
                //output += "r#\" \"#"
            } else {
                output += text;
            }

            if !del.is_some() {
                break;
            }

            i = *skips;

            if i <= self.text.len() {
                let (found, closing_index, del_content) =
                    &self.find_closing_delimeter(i, &del.unwrap());

                if !found {
                    // closing del not found , we push the symbol as normal text and continue

                    output.push_str(&del.unwrap().symbol);
                    output.push_str(del_content);
                    i = *closing_index + 1;
                    continue;
                }

                if i <= self.text.len() {
                    i = closing_index + del.unwrap().end_symbol.len();
                    let mut char_after_closing_del = "";
                    if i <= self.text.len() && del.unwrap().no_break {
                        char_after_closing_del = &self.get_slice(i, i + 1).unwrap_or("");
                    }

                    if char_after_closing_del != " "
                        && char_after_closing_del != ""
                        && del.unwrap().no_break
                    {
                        output.push_str("\"<span class=\"nobreak\">");
                    } else {
                        output.push_str("\"");
                    }
                    output.push_str(del.unwrap().left_replacement);
                    output.push_str("\"");

                    if del.unwrap().keep_delimiter {
                        output.push_str(&del.unwrap().symbol);
                    }

                    output.push_str(&del_content);

                    if del.unwrap().keep_delimiter {
                        output.push_str(&del.unwrap().end_symbol);
                    }
                    output.push_str("\"");
                    output.push_str(del.unwrap().right_replacement);
                    if del.unwrap().no_break
                        && char_after_closing_del != " "
                        && char_after_closing_del != ""
                    {
                        output.push_str("\"");
                        let mut string = "".to_string();
                        while i < self.text.len()
                            && self.get_char(i) != " "
                            && self.get_char(i) != ""
                        {
                            string.push_str(self.get_char(i).as_str());
                            i += 1;
                        }
                        let handled_string = self::ElementText::new(&string).handle_delimeters();
                        i += 1;
                        output.push_str(&handled_string);
                        output.push_str("\"</span>\"");
                    } else {
                        output.push_str("\"");
                        i += 1;
                    }
                }
            }
        }
        output
    }

    pub fn split_text(self) -> Vec<BlockChildType> {
        let mut i = 0;
        let mut j = 0;
        let mut output = Vec::<BlockChildType>::new();

        while i <= self.text.len() {
            let (del, skips, text) = &self.find_next_delimeter(i);

            output.push(BlockChildType::Text(TextCell {
                content: text.to_string(),
            }));

            if !del.is_some() {
                break;
            }

            i = *skips;

            if i <= self.text.len() {
                let (found, closing_index, del_content) =
                    &self.find_closing_delimeter(i, &del.unwrap());

                if !found {
                    // closing del not found , we push the symbol as normal text and continue

                    let mut last_child = output.pop().unwrap();
                    match last_child {
                        BlockChildType::Text(mut t) => {
                            t.content
                                .push_str(&format!("{}{}", &del.unwrap().symbol, del_content));
                            output.push(BlockChildType::Text(t))
                        }
                        _ => (),
                    }
                    i = *closing_index + 1;
                    continue;
                }

                if i <= self.text.len() {
                    i = closing_index + del.unwrap().end_symbol.len();

                    output.push(BlockChildType::Delimited(DelimitedCell {
                        terminal: del_content.to_owned(),
                        delimeter: del.unwrap().symbol.to_string(),
                    }));

                    i += 1;
                }
            }
        }
        output
    }

    fn find_next_delimeter(&self, mut i: usize) -> (Option<&DelimeterRules>, usize, String) {
        let mut found_symbol = "";
        let mut del: Option<&DelimeterRules> = None;
        let mut text = "".to_string();
        let symbols: Vec<&str> = DELIMETERS.iter().map(|d| d.symbol).collect();
        let mut has_multi_char = false;

        while i <= self.text.len() {
            let _del = DELIMETERS.iter().find(|d| {
                if i.checked_sub(d.symbol.len()).is_some() {
                    d.symbol
                        == &self
                            .text
                            .chars()
                            .take(i)
                            .skip(i - d.symbol.len())
                            .collect::<String>()
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
                    // pop the "\"
                    text.pop();

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
    ) -> (bool, usize, String) {
        let end_symbol = found_del.end_symbol;
        let mut found = false;
        let mut del_content = "".to_string();
        let symbols: Vec<&str> = DELIMETERS.iter().map(|d| d.symbol).collect();

        while i < self.text.len() {
            let is_found = end_symbol
                == &self
                    .text
                    .chars()
                    .take(i + end_symbol.len())
                    .skip(i)
                    .collect::<String>();
            if !is_found {
                if end_symbol
                    != &self
                        .text
                        .chars()
                        .take(i + end_symbol.len())
                        .skip(i)
                        .collect::<String>()
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
        self.text
            .chars()
            .nth(i)
            .unwrap_or(" ".chars().nth(0).unwrap())
            .to_string()
    }

    fn is_escaped(&self, i: usize) -> bool {
        i > 0 && self.get_char(i - 1) == "\\"
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
