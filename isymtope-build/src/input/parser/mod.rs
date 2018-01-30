pub mod token;
pub mod lexer;
pub mod parser;

use std::path::Path;
use std::io::{self, Read};
use std::fs;

use ast::*;


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
pub fn parse_str<'input>(input: &str) -> Result<Template, io::Error> {
    let lexer = lexer::lex(&input);

    let res = parser::parse_Template(lexer);
    if let Err(e) = res {
        println!("Parse error: {:?}", e);
        panic!(format!("Error parsing template from string."));
    }
    Ok(res.unwrap())
}

#[allow(dead_code)]
pub fn parse_file<'input>(path: &Path) -> Result<Template, io::Error> {
    let input = read_file(path)?;
    let lexer = lexer::lex(&input);

    let res = parser::parse_Template(lexer);
    if let Err(e) = res {
        println!("Parse error: {:?}", e);
        panic!(format!("Error parsing file {:?}", path));
    }
    Ok(res.unwrap())
}

#[cfg(test)]
mod tests {

    // #[test]
    // Disable as api is not currently supported in default scope
    #[allow(dead_code)]
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
                    action act1;
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

        // let mut tokens: Vec<Token> = Vec::new();
        // loop {
        // if let Some(Ok((start, tok, end))) = lexer.next() {
        // println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
        // tokens.push(tok);
        // } else {
        // println!("Done");
        // break;
        // }
        // }
        //

        let res = parser::parse_Template(lexer);
        println!("Result 1: {:?}", res);
        assert!(res.ok().is_some());
    }

    #[test]
    fn test_lexer2() {
        use parser::parser;

        let input = r#"
            component test() { h1 { } }
        "#;

        let lexer = super::parse(input);

        // let mut tokens: Vec<Token> = Vec::new();
        // loop {
        // if let Some(Ok((start, tok, end))) = lexer.next() {
        // println!("Got token {:?} ({:?} .. {:?})", tok, start, end);
        // tokens.push(tok);
        // } else {
        // println!("Done");
        // break;
        // }
        // }

        let res = parser::parse_Template(lexer);
        println!("Result 2: {:?}", res);
        assert!(res.ok().is_some());
    }

    // #[test]
    // Disable as api is not currently supported in default scope
    #[allow(dead_code)]
    fn test_lexer_file1() {
        let res = super::parse_file(::std::path::Path::new("./res/tests/test1.ism"));
        println!("Result for lexer test file1: {:?}", res);
        assert!(res.is_ok());
    }

    #[allow(dead_code)]
    fn test_lexer_file2() {
        let res = super::parse_file(::std::path::Path::new("./res/tests/test2.ism"));
        println!("Result for lexer test file2: {:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_lexer_file3() {
        let res = super::parse_file(::std::path::Path::new("./res/tests/test3.ism"));
        println!("Result for lexer test file3: {:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_lexer_file4() {
        let res = super::parse_file(::std::path::Path::new("./res/tests/test4.ism"));
        println!("Result for lexer test file4: {:?}", res);
        assert!(res.is_ok());
    }
}
