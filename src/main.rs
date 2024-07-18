use fnv::FnvHashMap;

fn main() {
    let input = r#"
# import ファイル名
# from ファイル名.dml import item_id, group_name
# Item Stone:
#  Name:
#    en: "Stone"
#    ja: "石"

#Item Firestone:
#  Name:
#    en: "Firestone"
#    ja: "火打石"

Group BasicMaterials:
  &Stone: -1.12
  &Firestone: -10
"#;

    let mut lexer = Lexer::new(input.to_string());
    lexer.generate_tokens();
    println!("{:?}", lexer.tokens);
}

#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Item,
    Group,
    Reference,
    Name,
    Colon,
    Int(i128),
    Float(f64),
    From,
    Import,
    Line(usize),
    Indent,
    String(String),
    Identifier(String),
}

struct Lexer {
    input: String,
    position: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_next(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_by(&mut self, n: usize) {
        self.position += n;
    }

    fn generate_tokens(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' | ':' | '#' | '&' | '"' | '-' => self.handle_special_characters(c),
                _ if c.is_numeric() => self.handle_number(),
                _ if c.is_alphabetic() => self.handle_identifier(),
                _ => self.advance(),
            }
        }
    }

    fn handle_special_characters(&mut self, c: char) {
        match c {
            ' ' => {
                if self.peek_next() == Some(' ') {
                    self.tokens.push(Token::Indent);
                    self.advance_by(2);
                } else {
                    self.advance();
                }
            }
            '\n' => {
                self.tokens.push(Token::Line(self.line));
                self.line += 1;
                self.advance();
            }
            ':' => {
                self.tokens.push(Token::Colon);
                self.advance();
            }
            '#' => {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            }
            '&' => {
                self.tokens.push(Token::Reference);
                self.advance();
            }
            '"' => {
                self.advance();
                let mut buffer = String::new();
                while let Some(c) = self.peek() {
                    if c != '"' {
                        buffer.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.advance();
                self.tokens.push(Token::String(buffer));
            }
            '-' => {
                if let Some(c) = self.peek_next() {
                    if c.is_numeric() {
                        self.handle_number();
                    } else {
                        self.advance();
                    }
                } else {
                    self.advance();
                }
            }
            _ => self.advance(),
        }
    }

    fn handle_number(&mut self) {
        let mut buffer = String::new();
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '-' {
                buffer.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if is_float {
            self.tokens.push(Token::Float(buffer.parse().unwrap()))
        } else {
            self.tokens.push(Token::Int(buffer.parse().unwrap()))
        }
    }

    fn handle_identifier(&mut self) {
        let mut buffer = String::new();
        while let Some(c) = self.peek() {
            if !c.is_whitespace() && c != ':' && c != '\n' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.tokens.push(match buffer.as_str() {
            "Item" => Token::Item,
            "Group" => Token::Group,
            "Name" => Token::Name,
            "from" => Token::From,
            "import" => Token::Import,
            _ => Token::Identifier(buffer),
        });
    }
}
