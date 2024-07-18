use fnv::FnvHashMap;
use std::{borrow::Cow, collections::HashMap};

fn main() {
    let input = r#"
import ファイル名
from ファイル名.dml import item_id, group_name
 Item Stone:
  Translations:
    en: "Stone"
    ja: "石"

Item Firestone:
  Translations:
    en: "Firestone"
    ja: "火打石"

Group BasicMaterials:
  &Stone: -1.12
  &Firestone: -10
"#;

    let mut lexer = Lexer::new(input.to_string());
    lexer.generate_tokens();
    println!("{:?}", lexer.tokens);

    let mut parser = Parser::new(lexer.tokens);

    // parser.parse();

    todo!("Propertiesの追加")
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Token {
    Item,
    Group,
    Reference,
    Translations,
    Colon,
    Int(i128),
    Float(f64),
    From,
    Import,
    Line(usize),
    Indent,
    String(String),
    Identifier(Cow<'static, str>),
}

impl Token {
    fn is_type(&self, token: &Token) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(token)
    }
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
                self.tokens.push(Token::String(buffer.into()));
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
            "Translations" => Token::Translations,
            "from" => Token::From,
            "import" => Token::Import,
            _ => Token::Identifier(buffer.into()),
        });
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    fn peek_by(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_by(&mut self, n: usize) {
        self.position += n;
    }

    fn is_next_token_type(&self, n: usize, token: Token) -> bool {
        if let Some(t) = self.peek_by(n) {
            t.is_type(&token)
        } else {
            false
        }
    }

    fn token_flow(&self, tokens: Vec<Token>) -> bool {
        let mut count = 1;
        for token in tokens.iter() {
            if self.peek_by(count) == Some(token) {
                count += 1;
            } else {
                return false;
            }
        }
        return true;
    }

    fn parse(&mut self) {
        while let Some(token) = self.peek() {
            match token {
                Token::Item => self.parse_item(),
                Token::Group => self.parse_group(),
                _ => self.advance(),
            }
        }
    }
    fn parse_item(&mut self) {
        self.advance();
        if self.token_flow(vec![
            Token::Identifier(Cow::Borrowed("")),
            Token::Colon,
            Token::Line(0),
        ]) {
            let mut item: HashMap<String, Item> = HashMap::new();
        }
    }

    fn parse_group(&mut self) {}
}

struct Item {
    translations: HashMap<String, String>,
}
