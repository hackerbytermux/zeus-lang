#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Identifier(String),
    Print,
    Input,
    LeftParen,
    RightParen,
    If,
    Then,
    Else,
    End,
    While,
    Do,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\n' => continue,
                '+' => return Token::Plus,
                '-' => return Token::Minus,
                '*' => return Token::Multiply,
                '/' => return Token::Divide,
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        return Token::Equal;
                    } else {
                        return Token::Assign;
                    }
                }
                '(' => return Token::LeftParen,
                ')' => return Token::RightParen,
                '>' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        return Token::GreaterEqual;
                    } else {
                        return Token::Greater;
                    }
                }
                '<' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        return Token::LessEqual;
                    } else {
                        return Token::Less;
                    }
                }
                '0'..='9' => {
                    let mut number = String::new();
                    number.push(ch);
                    while let Some(next_ch) = self.peek_char() {
                        if next_ch.is_digit(10) || next_ch == '.' {
                            number.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    return Token::Number(number.parse().unwrap());
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    identifier.push(ch);
                    while let Some(next_ch) = self.peek_char() {
                        if next_ch.is_alphabetic() || next_ch.is_digit(10) || next_ch == '_' {
                            identifier.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "print" => return Token::Print,
                        "input" => return Token::Input,
                        "if" => return Token::If,
                        "then" => return Token::Then,
                        "else" => return Token::Else,
                        "end" => return Token::End,
                        "while" => return Token::While,
                        "do" => return Token::Do,
                        _ => return Token::Identifier(identifier),
                    }
                }
                _ => panic!("Unexpected character: {}", ch),
            }
        }
        Token::EOF
    }
}