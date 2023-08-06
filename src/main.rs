#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Token {
    Illegal,
    Eof,

    // identifier and literals
    Ident(String),
    Int(String),

    // operators
    Assign,
    Plus,
    Minus,
    In,
    To,

    // delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Let,
}

struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let tok = match self.ch {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::In,
            '-' => {
                match self.peek() {
                    '>' => {
                        self.read_char();
                        Token::To
                    },
                    _ => Token::Minus,
                }
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "let" => Token::Let,
                    _ => Token::Ident(ident)
                }
            },
            '0'..='9' => {
                return Token::Int(self.read_num())
            },
            '\0' => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        return tok;
    }

    fn read_char(&mut self) {
        self.ch = if self.read_pos >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_pos).unwrap_or('\0')
        };
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek(&self) -> char {
        return if self.read_pos >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_pos).unwrap_or('\0')
        };
    }

    fn read_ident(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        return String::from(&self.input[pos..self.pos]);
    }

    fn read_num(&mut self) -> String{
        let pos = self.pos;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return String::from(&self.input[pos..self.pos]);
    }

    fn eat_whitespace(&mut self) { 
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tokens = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut l = Lexer::new(input.to_string());
        for tok in tokens {
            let next_tok = l.next_token();
            assert_eq!(next_tok, tok);
        }
    }

    #[test]
    fn test_next_token_define_variable() {
        let input = r#"let five = 5: I32;"#;

        let tokens = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::In,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.to_string());
        for tok in tokens {
            let next_tok = l.next_token();
            dbg!(&next_tok);
            assert_eq!(next_tok, tok);
        }
    }

    #[test]
    fn test_next_token_define_function() {
        let input = r#"
            let add(x,y) = { 
                x + y 
            }: (I32, I32) -> I32;
        "#;

        let tokens = [
            Token::Let,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::Assign,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::RBrace,
            Token::In,
            Token::LParen,
            Token::Ident(String::from("I32")),
            Token::Comma,
            Token::Ident(String::from("I32")),
            Token::RParen,
            Token::To,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.to_string());
        for tok in tokens {
            let next_tok = l.next_token();
            dbg!(&next_tok);
            assert_eq!(next_tok, tok);
        }
    }

    #[test]
    fn test_next_token_all() {
        let input = r#"
            let five = 5: I32;
            let ten = 10: I32;

            let add(x,y) = { 
                x + y 
            }: (I32, I32) -> I32;

            let result = add(five, ten): I32;
        "#;

        let tokens = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::In,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::In,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::Assign,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::RBrace,
            Token::In,
            Token::LParen,
            Token::Ident(String::from("I32")),
            Token::Comma,
            Token::Ident(String::from("I32")),
            Token::RParen,
            Token::To,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::In,
            Token::Ident(String::from("I32")),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.to_string());
        for tok in tokens {
            let next_tok = l.next_token();
            dbg!(&next_tok);
            assert_eq!(next_tok, tok);
        }
    }
}
