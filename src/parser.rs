use regex::Regex;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Atom(String),
    Not,
    And,
    Or,
    If,
    Iff,
    Then,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub enum AstNode {
    Atom(String),
    Not(Box<AstNode>),
    And(Box<AstNode>, Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    If(Box<AstNode>, Box<AstNode>),
    Iff(Box<AstNode>, Box<AstNode>),
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn parse_primary(&mut self) -> Option<AstNode> {
        match self.tokens.next() {
            Some(Token::If) => self.parse_conditional(),
            Some(Token::Iff) => self.parse_conditional(),
            Some(Token::Atom(atom)) => Some(AstNode::Atom(atom)),
            Some(Token::Not) => self.parse_not(),
            Some(Token::OpenParen) => self.parse_parenthesized_expr(),
            Some(Token::Then) => {
                panic!("Unexpected 'then' keyword without preceding 'if'");
            }
            _ => None,
        }
    }
    

    fn parse_not(&mut self) -> Option<AstNode> {
        match self.parse_primary() {
            Some(expr) => Some(AstNode::Not(Box::new(expr))),
            None => panic!("Expected expression after NOT"),
        }
    }

    fn parse_binary_op(
        &mut self,
        parse_left: fn(&mut Parser) -> Option<AstNode>,
        ops: &[Token],
    ) -> Option<AstNode> {
        let mut left = parse_left(self)?;
        while let Some(op) = self.tokens.peek().cloned() {
            if ops.contains(&op) {
                let tokens = self.tokens.by_ref();
                tokens.next();
                let right = parse_left(self)?;
                left = match op {
                    Token::And => AstNode::And(Box::new(left), Box::new(right)),
                    Token::Or => AstNode::Or(Box::new(left), Box::new(right)),
                    Token::If => AstNode::If(Box::new(left), Box::new(right)),
                    Token::Iff => AstNode::Iff(Box::new(left), Box::new(right)),
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }
        Some(left)
    }
    

    fn parse_expr(&mut self) -> Option<AstNode> {
        self.parse_binary_op(Parser::parse_primary, &[Token::And, Token::Or])
    }

    fn parse_conditional(&mut self) -> Option<AstNode> {
        let condition = self.parse_expr()?;
        if let Some(Token::Then) = self.tokens.next() {
            let consequence = self.parse_expr()?;
            Some(AstNode::If(Box::new(condition), Box::new(consequence)))
        } else {
            panic!("Expected 'then' keyword");
        }
    }    

    fn parse_parenthesized_expr(&mut self) -> Option<AstNode> {
        self.tokens.next();
        let expr = self.parse_expr()?;
        if let Some(Token::CloseParen) = self.tokens.peek() {
            self.tokens.next();
        } else {
            panic!("Missing closing parenthesis");
        }
        Some(expr)
    }

    pub fn parse(&mut self) -> Option<AstNode> {
        self.parse_expr()
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    let re = Regex::new(r"[a-zA-Z]+|not|and|or|if|iff|then|\(|\)").unwrap();
    let mut tokens = Vec::new();
    let mut last_token_was_atom = false; 
    let mut in_conditional = false;

    for mat in re.find_iter(input) {
        match mat.as_str() {
            "not" => {
                tokens.push(Token::Not);
                last_token_was_atom = false; 
            }
            "and" => {
                tokens.push(Token::And);
                last_token_was_atom = false;
            }
            "or" => {
                tokens.push(Token::Or);
                last_token_was_atom = false;
            }
            "if" => {
                tokens.push(Token::If);
                in_conditional = true;
                last_token_was_atom = false;
            }
            "iff" => {
                tokens.push(Token::Iff);
                in_conditional = true;
                last_token_was_atom = false;
            }
            "then" => {
                if !in_conditional {
                    panic!("Unexpected 'then' keyword without preceding 'if'");
                }
                tokens.push(Token::Then);
                last_token_was_atom = false;
            }
            "(" => {
                tokens.push(Token::OpenParen);
                last_token_was_atom = false;
            }
            ")" => {
                tokens.push(Token::CloseParen);
                last_token_was_atom = false;
            }
            atom => {
                tokens.push(Token::Atom(atom.to_string()));
                if last_token_was_atom {
                    panic!("Expected operator between atoms");
                }
                last_token_was_atom = true;
            }
        }
    }

    tokens
}