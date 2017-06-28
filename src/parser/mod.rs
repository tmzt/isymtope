
pub mod token;
pub mod ast;
pub mod lexer;
pub mod parser;

fn parse(input: &'static str) -> Box<Iterator<Item = token::Result<(usize, token::Token<'static>, usize)>>> {
    Box::new(lexer::lex(input))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lexer1() {
        use parser::token::Token;
        use parser::parser;

        let input = r#"
            component test { h1 { "hello" } }
            component test2(name) { p { name } }
            test2(name = "jack") { }
            test2(name = myname) { }
            test2(name = "jack", color = "black")
        "#;

        let mut tokens: Vec<Token<'static>> = Vec::new();
        let mut lexer = super::parse(input);

        loop {
            if let Some(Ok((start, tok, end))) = lexer.next() {
                println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
                tokens.push(tok);
            } else {
                println!("Done");
                break;
            }
        }

        let res = parser::parse_Template(tokens);
        println!("Result 1: {:?}", res);

    }

    #[test]
    fn test_lexer2() {
        use parser::token::Token;
        use parser::parser;

        let input = r#"
            component test { h1 { } }
        "#;

        let mut tokens: Vec<Token<'static>> = Vec::new();
        let mut lexer = super::parse(input);

        loop {
            if let Some(Ok((start, tok, end))) = lexer.next() {
                println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
                tokens.push(tok);
            } else {
                println!("Done");
                break;
            }
        }

        let res = parser::parse_Template(tokens);
        println!("Result 2: {:?}", res);

    }
}
