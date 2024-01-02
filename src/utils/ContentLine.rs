pub struct ContentLine {
    pub text: String,
}

pub struct DoubleDelimeter {
    pub symbol: &'static str,
    pub left_replacement: &'static str,
    pub right_replacement: &'static str,
}

impl ContentLine {
    pub fn new(text: &str) -> ContentLine {
        ContentLine {
            text: text.to_string(),
        }
    }

    pub fn add_delimeter(
        mut self,
        symbol: &str,
        left_replacement: &str,
        right_replacement: &str,
        no_break: bool,
        keep_delimeter: bool,
    ) -> Self {
        self.text = ContentLine::escape_chars(&self.text, symbol);

        let re = regex::Regex::new(&format!(
            r"\{}(.*?)\{}{}",
            symbol,
            symbol,
            if no_break { "(\\S*)" } else { "" }
        ))
        .unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                if caps.get(2).is_some() && caps.get(2).unwrap().len() > 0 {
                    // If the character after the second $ is not a space
                    format!(
                        "\"#<span class=\"nobreak\">{}r#\"{}{}{}\"#{}\"{}\"</span>r#\"",
                        left_replacement,
                        if keep_delimeter { symbol } else { "" },
                        &caps[1],
                        if keep_delimeter { symbol } else { "" },
                        right_replacement,
                        &caps[2]
                    )
                } else {
                    format!(
                        "\"#{}r#\"{}{}{}\"#{}r#\"",
                        left_replacement,
                        if keep_delimeter { symbol } else { "" },
                        &caps[1],
                        if keep_delimeter { symbol } else { "" },
                        right_replacement
                    )
                }
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, symbol);

        self
    }

    //very special case
    pub fn handle_math_block_back(mut self) -> Self {
        let re = regex::Regex::new(r"<MathBlock>(.*?)</MathBlock>").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("<MathBlock>r#\"$${}$$\"#</MathBlock>", &caps[1])
            })
            .to_string();
        self
    }

    pub fn escape_chars(text: &str, _char: &str) -> String {
        // handle escaped $
        let re = regex::Regex::new(&format!(r"\\\{}(.*?)\\\{}", _char, _char)).unwrap();
        let res = re
            .replace_all(&text, |caps: &regex::Captures| {
                format!("XescapedX{}XescapedX", &caps[1])
            })
            .to_string();

        res
    }
    pub fn un_escape_chars(text: &str, _char: &str) -> String {
        let re = regex::Regex::new(r"XescapedX(.*?)XescapedX").unwrap();
        let res = re
            .replace_all(&text, |caps: &regex::Captures| {
                format!("{}{}{}", _char, &caps[1], _char)
            })
            .to_string();

        res
    }
}
