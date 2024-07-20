use fnv::FnvHashMap;
use std::mem::discriminant;
use std::{borrow::Cow, collections::HashMap, fs};
fn main() {
    let file_path = "./test.dml";
    let mut input = String::new();
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("File Read OK");
            input = contents;
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }

    let mut lexer = Lexer::new(input);
    lexer.generate_tokens();
    println!("{:?}", lexer.tokens);

    let mut parser = Parser::new(lexer.tokens);

    parser.parse();

    // todo!("Propertiesの追加")
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Token {
    Item,
    Group,
    Reference,
    Colon,
    Int(i128),
    Float(f64),
    From,
    Import,
    NewLine,
    Indent,
    String(String),
    Identifier(Cow<'static, str>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TokenType {
    Item,
    Group,
    Reference,
    Colon,
    Int,
    Float,
    From,
    Import,
    Line,
    Indent,
    String,
    Identifier,
}

impl Token {
    // Tokenの種類をチェックするメソッド
    fn get_token_type(&self) -> TokenType {
        match self {
            Token::Item => TokenType::Item,
            Token::Group => TokenType::Group,
            Token::Reference => TokenType::Reference,
            Token::Colon => TokenType::Colon,
            Token::Int(_) => TokenType::Int,
            Token::Float(_) => TokenType::Float,
            Token::From => TokenType::From,
            Token::Import => TokenType::Import,
            Token::NewLine => TokenType::Line,
            Token::Indent => TokenType::Indent,
            Token::String(_) => TokenType::String,
            Token::Identifier(_) => TokenType::Identifier,
        }
    }
}

impl PartialEq<TokenType> for Token {
    // TokenTypeとTokenの比較を行うメソッド
    // TokenとTokenTypeは同じものとして扱う
    fn eq(&self, other: &TokenType) -> bool {
        match self {
            Token::Item => *other == TokenType::Item,
            Token::Group => *other == TokenType::Group,
            Token::Reference => *other == TokenType::Reference,
            Token::Colon => *other == TokenType::Colon,
            Token::Int(_) => *other == TokenType::Int,
            Token::Float(_) => *other == TokenType::Float,
            Token::From => *other == TokenType::From,
            Token::Import => *other == TokenType::Import,
            Token::NewLine => *other == TokenType::Line,
            Token::Indent => *other == TokenType::Indent,
            Token::String(_) => *other == TokenType::String,
            Token::Identifier(_) => *other == TokenType::Identifier,
        }
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
                self.tokens.push(Token::NewLine);
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
            "from" => Token::From,
            "import" => Token::Import,
            _ => Token::Identifier(buffer.into()),
        });
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    indent_level: usize,
    line: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            indent_level: 0,
            line: 1,
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

    fn token_flow(&self, token_types: Vec<TokenType>) -> bool {
        for (i, token_type) in token_types.iter().enumerate() {
            // self.tokens から現在の位置 + i にあるトークンのトークンタイプを取得
            if let Some(token) = self.tokens.get(self.position + i) {
                // トークンタイプが一致するか確認
                if token != token_type {
                    // panic!("not match token type: {:?}", token);
                    return false;
                }
            } else {
                // トークンが存在しない場合は、false を返す
                return false;
            }
        }
        true
    }

    fn parse_error(&self, message: &str) {
        panic!("Parse Error: line[{}], message: {}", self.line, message);
    }

    /// トークンのパターンが正しいかどうかを確認するメソッド
    fn check_group_statement(&mut self) -> bool {
        if self.token_flow(vec![
            TokenType::Identifier,
            TokenType::Colon,
            TokenType::Line,
        ]) {
            self.advance_by(3);
            self.line += 1;
            return true;
        } else {
            return false;
        }
    }

    fn check_item_statement(&mut self) -> bool {
        if self.token_flow(vec![
            TokenType::Identifier,
            TokenType::Colon,
            TokenType::Line,
        ]) {
            self.advance_by(3);
            self.line += 1;
            return true;
        } else {
            return false;
        }
    }

    fn parse(&mut self) {
        println!("Parsing Started");
        while let Some(token) = self.peek() {
            match token {
                Token::Item => self.parse_item(),
                Token::Group => self.parse_group(),
                Token::Indent => self.parse_indent(),
                Token::NewLine => self.parse_new_line(),
                _ => {
                    // println!("{:?}", &token);
                    self.advance();
                }
            }
        }
        println!("Parsing Finished");
    }

    /// Itemをパースするメソッド
    fn parse_item(&mut self) {
        self.advance();
        if self.check_item_statement() {
            println!("Item Found: id = {:?}", self.peek().unwrap());
        } else {
            self.parse_error("Item Statement is invalid");
        }
    }

    /// Groupをパースするメソッド
    fn parse_group(&mut self) {
        self.advance();
        if self.check_group_statement() {
            println!("Group Found: id = {:?}", self.peek().unwrap());
        } else {
            self.parse_error("Group Statement is invalid");
        }
    }

    /// インデントをパースするメソッド
    fn parse_indent(&mut self) {
        self.indent_level += 1;
        self.advance();
    }

    /// 改行をパースするメソッド
    fn parse_new_line(&mut self) {
        self.indent_level = 0;
        self.line += 1;
        self.advance();
    }
}
