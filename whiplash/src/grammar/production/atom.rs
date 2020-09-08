use std::fmt;
use crate::parser::node::NodeType;
use crate::lexical_analyser::token::{Token, TokenType};

#[derive(Clone)]
pub enum Atom {
    Var(NodeType),
    Tok(Token),
    TokType(TokenType),
    Epsilon
}
impl Atom {
    /// Returns value inside token if Atom variant is token; else returns None
    pub fn get_token_value(&self) -> Option<String> {
        if let Self::Tok(token) = self {
            return Some(token.value.clone());
        }

        None
    }

    pub fn from_token(tok: String) -> Atom {
        Atom::Tok(Token::from(tok))
    }
    
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Atom::Var(a) => write!(f, "{:?}", a),
            Atom::Tok(a) => write!(f, "{:?}", a),
            Atom::TokType(a) => write!(f, "{:?}", a),
            Atom::Epsilon => write!(f, "ε")
        }
    }
}

pub struct Atoms {
    pub vals: Vec<Atom>
}

impl Atoms {
    pub fn from(v: Vec<Atom>) -> Atoms {
        Atoms {
            vals: v
        }
    }

    pub fn from_single_token(tok: String) -> Atoms {
        Atoms {
            vals: vec![Atom::from_token(tok)]
        }
    }

    pub fn from_single_atom(a: Atom) -> Atoms {
        Atoms {
            vals: vec![a]
        }
    }
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.vals.iter().fold(
                String::new(), 
                |acc, atom| acc + &format!("{:?}", &atom)[..] + " "
            )
        )
    }
}