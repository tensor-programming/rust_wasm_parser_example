struct Parser {
    pos: usize,
    input: String,
}

pub fn parse(source: String) -> String {
    Parser {
        pos: 0,
        input: source,
    }.parse_lines()
}

impl Parser {
    fn parse_lines(&mut self) -> String {
        let mut result = String::new();

        loop {
            self.consume_whitespace();
            if self.end_of_line() {
                break;
            }

            result.push_str(&self.parse_line());
        }

        result
    }

    fn parse_line(&mut self) -> String {
        match self.next_char() {
            '#' => self.parse_title(),
            '-' => {
                if char::is_whitespace(self.input[self.pos + 1..].chars().next().unwrap()) {
                    self.parse_list()
                } else {
                    self.parse_text()
                }
            }
            _ => self.parse_text(),
        }
    }

    fn parse_list(&mut self) -> String {
        self.consume_char();
        self.consume_whitespace();

        let text = self.parse_text();
        create_html_element("li".to_string(), text)
    }

    fn parse_title(&mut self) -> String {
        let pound = self.consume_while(|c| c == '#');
        self.consume_whitespace();
        let text = self.parse_text();

        create_html_element(format!("h{}", pound.len()), text)
    }

    fn parse_text(&mut self) -> String {
        self.consume_while(|c| !is_newline(c))
    }

    fn end_of_line(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.end_of_line() && cond(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

fn create_html_element(tag_name: String, text: String) -> String {
    format!("<{}>{}</{}>", tag_name, text, tag_name)
}

fn is_newline(c: char) -> bool {
    c == '\n'
}
