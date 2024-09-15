use std::collections::HashMap;

use crate::{lexer::Token, parser::Expr};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Expr>) {
        for statement in statements {
            self.eval(statement);
        }
    }

    fn eval(&mut self, expr: Expr) {
        match expr {
            Expr::Number(n) => (),
            Expr::Variable(name) => (),
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expr(*left);
                let right_val = self.eval_expr(*right);
                match op {
                    Token::Plus => println!("{}", left_val + right_val),
                    Token::Minus => println!("{}", left_val - right_val),
                    Token::Multiply => println!("{}", left_val * right_val),
                    Token::Divide => println!("{}", left_val / right_val),
                    Token::Greater => println!("{}", left_val > right_val),
                    Token::Less => println!("{}", left_val < right_val),
                    Token::GreaterEqual => println!("{}", left_val >= right_val),
                    Token::LessEqual => println!("{}", left_val <= right_val),
                    Token::Equal => println!("{}", left_val == right_val),
                    _ => panic!("Unexpected binary operator: {:?}", op),
                }
            }
            Expr::Assign { name, value } => {
                let val = self.eval_expr(*value);
                self.variables.insert(name, val);
            }
            Expr::Print(expr) => {
                let val = self.eval_expr(*expr);
                println!("{}", val);
            }
            Expr::Input(expr) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let val: f64 = input.trim().parse().unwrap();
                self.variables.insert(expr, val);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(*condition);
                if cond_val != 0.0 {
                    for stmt in then_branch {
                        self.eval(stmt);
                    }
                } else {
                    for stmt in else_branch {
                        self.eval(stmt);
                    }
                }
            },
            Expr::While { condition, body } => {
                let contition = condition.clone();
                while self.eval_expr(*contition.clone()) != 0.0 {
                    let body = body.clone();
                    for stmt in body.clone() {
                        self.eval(stmt);
                    }
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> f64 {
        match expr {
            Expr::Number(n) => n,
            Expr::Variable(name) => {
                *self.variables.get(&name).unwrap_or_else(|| panic!("Undefined variable: {}", name))
            }
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expr(*left);
                let right_val = self.eval_expr(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Multiply => left_val * right_val,
                    Token::Divide => left_val / right_val,
                    Token::Greater => (left_val > right_val) as i32 as f64,
                    Token::Less => (left_val < right_val) as i32 as f64,
                    Token::GreaterEqual => (left_val >= right_val) as i32 as f64,
                    Token::LessEqual => (left_val <= right_val) as i32 as f64,
                    Token::Equal => (left_val == right_val) as i32 as f64,
                    _ => panic!("Unexpected binary operator: {:?}", op),
                }
            }
            Expr::Assign { name: _, value: _ } => panic!("Assignments should be handled in eval"),
            Expr::Print(_) => panic!("Print should be handled in eval"),
            Expr::Input(_) => panic!("Input should be handled in eval"),
            Expr::If { .. } => panic!("If should be handled in eval"),
            Expr::While { .. } => panic!("While should be handled in eval"),
        }
    }
}