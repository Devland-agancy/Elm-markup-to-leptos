pub struct ContentLine {
    pub text: String,
}

impl ContentLine {
    pub fn new(text: &str) -> ContentLine {
        ContentLine {
            text: text.to_string(),
        }
    }

    pub fn handle_bold(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "*");

        let re = regex::Regex::new(r"\*(.*?)\*").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span bold=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "*");

        self
    }

    pub fn handle_italic(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "_");

        let re = regex::Regex::new(r"\_(.*?)\_").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                format!("\"#<Span italic=true>r#\"{}\"#</Span>r#\"", &caps[1])
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "_");

        self
    }

    pub fn handle_math_block(mut self) -> Self {
        let re = regex::Regex::new(r"\$\$(.*?)\$\$").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                // $$ is not included here as it will cause issues with handle_math pub fn , after we call handle_math we add $$ back
                format!("\"#<MathBlock>{}</MathBlock>r#\"", &caps[1])
            })
            .to_string();
        self
    }

    pub fn handle_math(mut self) -> Self {
        self.text = ContentLine::escape_chars(&self.text, "$");

        let re = regex::Regex::new(r"\$(.*?)\$(\S*)").unwrap();
        self.text = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                if caps.get(2).is_some() && caps.get(2).unwrap().len() > 0 {
                    // If the character after the second $ is not a space
                    format!(
                        "\"#<span class=\"nobreak\"><Math>r#\"${}$\"#</Math>\"{}\"</span>r#\"",
                        &caps[1], &caps[2]
                    )
                } else {
                    format!("\"#<Math>r#\"${}$\"#</Math>r#\"", &caps[1])
                }
            })
            .to_string();

        self.text = ContentLine::un_escape_chars(&self.text, "$");

        self
    }

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
