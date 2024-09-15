use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Print(Box<Expr>),
    Input(String),
    If {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Vec<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Expected {:?}, found {:?}", token, self.current_token);
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> Expr {
        match self.current_token {
            Token::Identifier(ref name) => {
                let name_clone = name.clone(); // Clone the name here
                self.eat(Token::Identifier(name_clone.clone()));
                if self.current_token == Token::Assign {
                    self.eat(Token::Assign);
                    let value = self.parse_expr();
                    Expr::Assign {
                        name: name_clone,
                        value: Box::new(value),
                    }
                } else {
                    panic!("Expected assignment after identifier");
                }
            }
            Token::Print => {
                self.eat(Token::Print);
                let expr = self.parse_expr();
                Expr::Print(Box::new(expr))
            }
            Token::Input => {
                self.eat(Token::Input);
                let expr = self.parse_factor();
                match expr {
                    Expr::Variable(name) => Expr::Input(name),
                    _ => panic!("Expected variable after input"),
                }
            }
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_if(&mut self) -> Expr {
        self.eat(Token::If);
        let condition = self.parse_expr();
        self.eat(Token::Then);
        let mut then_branch = Vec::new();
        while self.current_token != Token::Else && self.current_token != Token::End {
            then_branch.push(self.parse_statement());
        }
        let else_branch = if self.current_token == Token::Else {
            self.eat(Token::Else);
            let mut else_branch = Vec::new();
            while self.current_token != Token::End {
                else_branch.push(self.parse_statement());
            }
            self.eat(Token::End);
            else_branch
        } else {
            self.eat(Token::End);
            Vec::new()
        };
        Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }
    }


    fn parse_while(&mut self) -> Expr {
        self.eat(Token::While);
        let condition = self.parse_expr();
        self.eat(Token::Do);
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.eat(Token::End);
        Expr::While {
            condition: Box::new(condition),
            body,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while matches!(
            self.current_token,
            Token::Plus | Token::Minus | Token::Greater | Token::Less | Token::GreaterEqual
                | Token::LessEqual | Token::Equal
        ) {
            let op = self.current_token.clone();
            self.eat(op.clone());
            let right = self.parse_term();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.eat(op.clone());
            let right = self.parse_factor();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current_token {
            Token::Number(n) => {
                self.eat(Token::Number(n));
                Expr::Number(n)
            }
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()));
                Expr::Variable(name.clone())
            }
            Token::LeftParen => {
                self.eat(Token::LeftParen);
                let expr = self.parse_expr();
                self.eat(Token::RightParen);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
}