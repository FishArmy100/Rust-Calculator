
pub enum TokenType
{
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
    Equles,
    OpenParen,
    CloseParen,
    Comma,
    SemiColon,
}

impl std::string::ToString for TokenType
{
    fn to_string(&self) -> String 
    {
        match self 
        {
            TokenType::Number(num) => num.to_string(),
            TokenType::Identifier(id) => id.clone(),
            TokenType::Plus => String::from("+"),
            TokenType::Minus => String::from("-"),
            TokenType::Star => String::from("*"),
            TokenType::Slash => String::from("/"),
            TokenType::Carrot => String::from("^"),
            TokenType::Equles => String::from("="),
            TokenType::OpenParen => String::from("("),
            TokenType::CloseParen => String::from(")"),
            TokenType::Comma => String::from(","),
            TokenType::SemiColon => String::from(";"),
        }
    }
}

pub struct Token
{
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize, 
}

impl std::string::ToString for Token
{
    fn to_string(&self) -> String 
    {
        format!("[{}, {}]: {}", self.line, self.column, &self.token_type.to_string())
    }
}