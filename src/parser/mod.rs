pub mod token;
pub mod ast;
pub mod loc;
pub mod store;
pub mod api;
pub mod lexer;
pub mod parser;
pub mod util;

use std::path::Path;
use std::io::{self, Read};
use std::fs;

type TokenIter = Box<Iterator<Item = token::Result<(usize, token::Token, usize)>>>;

#[allow(dead_code)]
pub fn parse(input: &'static str) -> TokenIter {
    Box::new(lexer::lex(input))
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

#[allow(dead_code)]
pub fn parse_file<'input>(path: &Path) -> Result<ast::Template, io::Error> {
    let input = read_file(path)?;
    let lexer = lexer::lex(&input);

    let res = parser::parse_Template(lexer);
    if let Err(e) = res {
        println!("Parse error: {:?}", e);

        panic!("Error parsing file");
    }
    Ok(res.unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lexer1() {
        use parser::parser;

        let input = r#"
            use html;

            store {
                let counter;

                api {
                    resource images {
                        get;
                    }
                    get;
                }

                scope1 {
                    action act1
                }
            }

            component test { h1 { "hello" } }
            component test2(name) { p { { name } } }
            component test3(name, color) { p(style = color) { { name } } }
            test2(name = "jack") { }
            test2(name = myname) { }
            test2(name = "jack", color = "black");
        "#;

        let lexer = super::parse(input);

        /*
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            if let Some(Ok((start, tok, end))) = lexer.next() {
                println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
                tokens.push(tok);
            } else {
                println!("Done");
                break;
            }
        }
        */

        let res = parser::parse_Template(lexer);
        println!("Result 1: {:?}", res);
        assert!(res.ok().is_some());
    }

    #[test]
    fn test_lexer2() {
        use parser::parser;

        let input = r#"
            component test { h1 { } }
        "#;

        let lexer = super::parse(input);

        /*
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            if let Some(Ok((start, tok, end))) = lexer.next() {
                println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
                tokens.push(tok);
            } else {
                println!("Done");
                break;
            }
        }*/

        let res = parser::parse_Template(lexer);
        println!("Result 2: {:?}", res);
        assert!(res.ok().is_some());
    }

    #[test]
    fn test_lexer3() {
        let res = super::parse_file(::std::path::Path::new("./res/tests/test1.ism"));
        println!("Result 3: {:?}", res);
        assert!(res.is_ok());
    }

}
