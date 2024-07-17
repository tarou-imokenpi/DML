fn main() {
    let input = r#"
import ファイル名
from ファイル名.dml import item_id, group_name
Item Stone:
  Name:
    en: Stone
    ja: 石

Item Firestone:
  Name:
    en: Firestone
    ja: 火打石

Group BasicMaterials:
  &Stone: 1
  &Firestone: 1
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
    Number(i128),
    From,
    Import,
    Line(usize),
    Indent,
    String(String),
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
        if self.position >= self.input.len() {
            return None;
        }

        self.input.chars().nth(self.position)
    }

    fn peek_next(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            return None;
        }

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
                    self.advance();
                    self.line += 1;
                }
                ':' => {
                    self.tokens.push(Token::Colon);
                    self.advance();
                }
                '#' => {
                    self.advance();
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
                _ => {
                    if c.is_numeric() {
                        let mut buffer = String::new();
                        while let Some(c) = self.peek() {
                            if c.is_numeric() {
                                buffer.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.tokens.push(Token::Number(buffer.parse().unwrap()));
                        continue;
                    }

                    if c.is_alphabetic() {
                        let mut buffer = String::new();
                        while let Some(c) = self.peek() {
                            if !c.is_whitespace() && c != ':' && c != '\n' {
                                buffer.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        match buffer.as_str() {
                            "Item" => self.tokens.push(Token::Item),
                            "Group" => self.tokens.push(Token::Group),
                            "Name" => self.tokens.push(Token::Name),
                            "from" => self.tokens.push(Token::From),
                            "import" => self.tokens.push(Token::Import),
                            _ => self.tokens.push(Token::String(buffer)),
                        }
                    } else {
                        self.advance();
                    }
                }
            }
        }
    }
}
