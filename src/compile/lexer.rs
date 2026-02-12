pub enum Token {
    Identifier(String),
    Keyword(String),
    Punctuator(String),
    Operator(String),
    Literal(String)
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Lexeme {
    pub data: String,
    pub line: usize
}

pub fn scan(source: String) -> Vec<Lexeme> {
    let mut is_string = false;
    let mut escaped = false;
    let chars = source.chars();

    let mut lexemes = Vec::new();
    let mut lexeme = Lexeme::default();

    for c in chars {
        if !is_string {
            lexeme.data.push(c);
            if c == '"' {
                is_string = true;
                escaped = false;
                continue;
            }

            if c == ' ' {
                lexemes.push(lexeme.clone());
            }
        } else {
            if c == '\\' {
                if escaped {
                    lexeme.data.push(c);
                }
                escaped = !escaped;
                continue;
            }
            
            lexeme.data.push(c);
            
            if c == '"' {
                lexemes.push(lexeme.clone());
                is_string = false;
            }
        }
    }

    lexemes
}