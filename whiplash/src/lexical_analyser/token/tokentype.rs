use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
#[derive(PartialEq, Copy, Clone, EnumIter)]
pub enum TokenType {
    OP,         // Operator
    NUM,        // Numeric value
    BOOL,       // Boolean
    KEYWORD,    // Keyword
    ID,         // Identifier
    LIT,        // Any kind of literal
    SYM,        // Symbol (such as ',' and ':')
    PAR,        // Parenthesis
}
