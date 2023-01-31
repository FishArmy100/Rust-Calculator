use crate::lexing::string_reader::StringReader;
use crate::lexing::token::*;

pub struct LexerError
{
    pub message: String,
    pub line: usize,
    pub column: usize
}

impl std::string::ToString for LexerError
{
    fn to_string(&self) -> String
    {
        format!("{}: line {}, column {}", self.message, self.line, self.column)
    }
}

fn scan_identifier(reader: &mut StringReader) -> Option<Token>
{
    if reader.current().is_alphabetic()
    {
        let line = reader.line();
        let column = reader.column();

        let mut length: usize = 0;
        while let Some(c) = reader.peek(length)
        {
            if c.is_alphanumeric()
            {
                length += 1;
            }
            else
            {
                break;
            }
        }

        Some(Token 
            {
                token_type: TokenType::Identifier(reader.advance(length)),
                line,
                column,
            })
    }
    else  
    {
        None
    }

}

fn skip_whitespace(reader: &mut StringReader)
{
    while !reader.at_end() && reader.current().is_whitespace()
    {
        reader.advance(1);
    }
}

fn scan_number(reader: &mut StringReader) -> Option<Token>
{
    let mut has_dot = false;
    let line = reader.line();
    let column = reader.column();

    if reader.current().is_numeric()
    {
        let mut length = 0;
        while let Some(c) = reader.peek(length)
        {
            if !c.is_numeric() && !(c == '.')
            {
                break;
            }

            if c == '.'
            {
                if has_dot { break; }
                has_dot = true;
            }

            length += 1;
        }

        let number = reader.advance(length).parse().unwrap();
        Some(Token {
            token_type: TokenType::Number(number),
            line: line,
            column: column
        })
    }
    else
    {
        None
    }
}

fn scan_symbol(reader: &mut StringReader) -> Option<Token>
{
    let found_type = match reader.current()
    {
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '*' => Some(TokenType::Star),
        '/' => Some(TokenType::Slash),
        '^' => Some(TokenType::Carrot),
        ',' => Some(TokenType::Comma),
        '=' => Some(TokenType::Equles),
        '(' => Some(TokenType::OpenParen),
        ')' => Some(TokenType::CloseParen),
        ';' => Some(TokenType::SemiColon),
        _ => None
    };

    match found_type 
    {
        Some(token_type) => 
        {
            let line = reader.line();
            let column = reader.column();
            reader.advance(1);
            Some(Token{
                token_type: token_type,
                line: line,
                column: column
            })
        }
        None => None
    }
}

pub type LexerResult = Result<Vec<Token>, Vec<LexerError>>;

fn run_lexer_step(reader: &mut StringReader) -> Result<Token, LexerError>
{
    skip_whitespace(reader);

    if let Some(tok) = scan_identifier(reader)
    {
        Ok(tok)
    }
    else if let Some(tok) = scan_symbol(reader)
    {
        Ok(tok)
    }
    else if let Some(tok) = scan_number(reader)
    {
        Ok(tok)
    }
    else
    {
        let error = LexerError
        {
            message: format!("Unknown token: {}", reader.current()),
            column: reader.column(),
            line: reader.line()
        };
        _ = reader.advance(1);
        Err(error)
    }
}

pub fn run_lexer(reader: &mut StringReader) -> LexerResult
{
    let mut errors: Vec<LexerError> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    while !reader.at_end()
    {
        match run_lexer_step(reader)
        {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error)
        };
    }

    if errors.len() > 0
    {
        Err(errors)
    }
    else
    {
        Ok(tokens)
    }

}