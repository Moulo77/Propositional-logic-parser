use regex::Regex;
use std::iter::Peekable;
use std::vec::IntoIter; 

//Les types de token
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

//Les types de noeuds de l'AST (abstract syntax tree)
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
    tokens: Peekable<IntoIter<Token>>, // Stocke les tokens en cours de traitement.
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(), // Initialise un iterateur
        }
    }

    // Analyse les expressions primaires en fonction du token.
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

    // Analyser l'opérateur NOT et verifie qu'il est suivi d'un atome dans l'AST.
    fn parse_not(&mut self) -> Option<AstNode> {
        match self.parse_primary() {
            Some(expr) => Some(AstNode::Not(Box::new(expr))),
            None => panic!("Expected expression after NOT"),
        }
    }

    // Analyser les opérations binaires.
    fn parse_binary_op( &mut self, parse_left: fn(&mut Parser) -> Option<AstNode>, ops: &[Token],) -> Option<AstNode> {
        let mut left = parse_left(self)?; // Recupere le noeud voisin de gauche
        while let Some(op) = self.tokens.peek().cloned() {
            if ops.contains(&op) {
                let tokens = self.tokens.by_ref();
                tokens.next();
                let right = parse_left(self)?; // Recupere le noeud voisin de droite
                left = match op { // Construit le noeud de l'AST correspondant à l'opération binaire.
                    Token::And => AstNode::And(Box::new(left), Box::new(right)),
                    Token::Or => AstNode::Or(Box::new(left), Box::new(right)),
                    Token::If => AstNode::If(Box::new(left), Box::new(right)),
                    Token::Iff => AstNode::Iff(Box::new(left), Box::new(right)),
                    _ => unreachable!(), // Erreur si un token non géré est rencontré.
                };
            } else {
                break; // Arrête la boucle si le token n'est pas un opérateur géré.
            }
        }
        Some(left) // Retourne le nœud d'AST résultant.
    }

    // Analyser une expression.
    fn parse_expr(&mut self) -> Option<AstNode> {
        self.parse_binary_op(Parser::parse_primary, &[Token::And, Token::Or])
    }

    // Analyser une expression conditionnelle IF.
    fn parse_conditional(&mut self) -> Option<AstNode> {
        let condition = self.parse_expr()?;
        if let Some(Token::Then) = self.tokens.next() { // Vérifie si le token suivant est "then".
            let consequence = self.parse_expr()?;
            Some(AstNode::If(Box::new(condition), Box::new(consequence))) // Retourne un nœud de l'AST pour l'expression conditionnelle.
        } else {
            panic!("Expected 'then' keyword"); // Erreur si "then" est manquant après une expression conditionnelle.
        }
    }

    // Analyser une expression entre parenthèses.
    fn parse_parenthesized_expr(&mut self) -> Option<AstNode> {
        self.tokens.next(); // Avance au token suivant, qui doit être une expression.
        let expr = self.parse_expr()?;
        if let Some(Token::CloseParen) = self.tokens.peek() { // Vérifie si le token suivant est une parenthèse fermante.
            self.tokens.next();
        } else {
            panic!("Missing closing parenthesis"); // Erreur si une parenthèse fermante est manquante.
        }
        Some(expr) // Retourne l'expression analysée.
    }

    // Fonction principale pour analyser l'entrée et construire l'AST.
    pub fn parse(&mut self) -> Option<AstNode> {
        self.parse_expr()
    }
}

// Fonction pour le lexing de l'entrée et la production de tokens.
pub fn lex(input: &str) -> Vec<Token> {
    let re = Regex::new(r"[a-zA-Z]+|not|and|or|if|iff|then|\(|\)").unwrap();
    let mut tokens = Vec::new(); // Initialise un vecteur pour stocker les tokens.
    let mut last_token_was_atom = false; // Vrai si le dernier token était un atome.
    let mut in_conditional = false; // Vrai si on est à l'intérieur d'une expression conditionnelle.

    // Parcourt l'entrée en utilisant le pattern regex.
    for mat in re.find_iter(input) { 
        // Ajoute un token selon l'entrée recu
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
                // Verifie si on se trouve dans une boucle conditionnel
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
                // Verifie que le dernier token n'est pas un atom
                if last_token_was_atom {
                    panic!("Expected operator between atoms");
                }
                last_token_was_atom = true;
            }
        }
    }

    tokens // Retourne le vecteur de tokens produit.
}
