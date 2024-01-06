pub struct ContentLine {
    pub text: String,
}
struct DelimeterRules {
    symbol: &'static str,
    left_replacement: &'static str,
    right_replacement: &'static str,
    no_break: bool,
    keep_delimiter: bool,
}

const DELIMETERS: [DelimeterRules; 5] = [
    DelimeterRules {
        symbol: "*",
        left_replacement: "<Span bold=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "__",
        left_replacement: "<Paragragh Align::Center><Span italic=true>",
        right_replacement: "</Span></Paragraph>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "_",
        left_replacement: "<Span italic=true>",
        right_replacement: "</Span>",
        no_break: false,
        keep_delimiter: false,
    },
    DelimeterRules {
        symbol: "$$",
        left_replacement: "<MathBlock>",
        right_replacement: "</MathBlock>",
        no_break: true,
        keep_delimiter: true,
    },
    DelimeterRules {
        symbol: "$",
        left_replacement: "<Math>",
        right_replacement: "</Math>",
        no_break: true,
        keep_delimiter: true,
    },
];

impl ContentLine {
    pub fn new(text: &str) -> ContentLine {
        ContentLine {
            text: text.to_string(),
        }
    }

    pub fn handle_delimeters(self) -> String {
        let symbols: Vec<&str> = DELIMETERS.iter().map(|d| d.symbol).collect();
        let mut i = 0;
        let mut j = 0;

        let mut output = String::new();

        let mut found_symbol: String = "".to_string();

        while i < self.text.len() {
            while i < self.text.len()
                && ((i > 0 && self.get_char(i - 1) == "\\")
                    || !symbols.contains(&self.get_char(i).as_str()))
            {
                output.push_str(&self.get_char(i).as_str());
                i = i + 1;
                j = i;
            }

            if i < self.text.len() {
                let c = &self.get_char(i)[..];
                let next_char = &self.get_char(i + 1)[..];

                if next_char == c {
                    found_symbol = c.to_string() + c;

                    i = i + 2;
                    j = i;
                    while i < self.text.len()
                        && (self.get_char(i - 1) == "\\"
                            || (i < self.text.len() - 1
                                && found_symbol != self.get_slice(i, i + 2).unwrap()))
                    {
                        i = i + 1;
                    }
                    if i == self.text.len() - 1 {
                        // not found , we add one so that  i < self.text.len()
                        i = i + 1
                    }
                } else {
                    found_symbol = c.to_string();
                    i = i + 1;
                    j = i;
                    while i < self.text.len()
                        && (self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_char(i).as_str())
                    {
                        i = i + 1;
                    }
                }

                if i < self.text.len() {
                    let del = self.get_delimeter(&found_symbol);
                    let mut char_after_closing_del = "";
                    if i + 1 < self.text.len() && del.no_break {
                        char_after_closing_del = self.get_slice(i + 1, i + 2).unwrap();
                        if next_char == c && i + 2 < self.text.len() {
                            char_after_closing_del = self.get_slice(i + 2, i + 3).unwrap();
                        }
                    }
                    if char_after_closing_del != " " && char_after_closing_del != "" && del.no_break
                    {
                        output.push_str("\"#<span class=\"nobreak\">");
                    } else {
                        output.push_str("\"#");
                    }
                    output.push_str(del.left_replacement);
                    output.push_str("r#\"");
                    if del.keep_delimiter {
                        output.push_str(&found_symbol);
                    }
                    i = j;
                    if next_char == c {
                        while self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_slice(i, i + 2).unwrap()
                        {
                            output.push_str(self.get_char(i).as_str());
                            i = i + 1;
                        }
                        i = i + 2
                    } else {
                        while self.get_char(i - 1) == "\\"
                            || found_symbol != self.get_char(i).as_str()
                        {
                            output.push_str(self.get_char(i).as_str());
                            i = i + 1
                        }
                        i = i + 1
                    }

                    if del.keep_delimiter {
                        output.push_str(&found_symbol);
                    }
                    output.push_str("\"#");
                    output.push_str(del.right_replacement);
                    if del.no_break && char_after_closing_del != " " && char_after_closing_del != ""
                    {
                        output.push_str("</span>r#\"");
                    } else {
                        output.push_str("r#\"");
                    }
                } else {
                    // closing del not found , we push the symbol as normal text and continue
                    output.push_str(&found_symbol);
                    i = j;
                }
            }
        }

        output
    }

    fn get_delimeter(&self, symbol: &str) -> &DelimeterRules {
        DELIMETERS.iter().find(|d| d.symbol == symbol).unwrap()
    }

    fn get_char(&self, i: usize) -> String {
        self.text
            .chars()
            .nth(i)
            .unwrap_or(" ".chars().nth(0).unwrap())
            .to_string()
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
