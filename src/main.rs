
mod lexing;
use lexing::string_reader::StringReader;
use lexing::lexer::*;

fn main() {
    let src = &String::from("func(x) = x^2 + 4.556*x - 5;");

    let mut reader = StringReader::create(src);
    
    let result = run_lexer(&mut reader);

    match result
    {
        Ok(tokens) => 
        {
            let token_strings: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
            for token in token_strings
            {
                println!("{}", token);
            }
        },
        Err(errors) => 
        {
            let error_strings: Vec<String> = errors.iter().map(|t| t.to_string()).collect();
            for error in error_strings
            {
                println!("{}", error);
            }
        }
    }
}
