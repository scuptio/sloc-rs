use std::fs;
use std::path::Path;
use logos::Logos;


// token type
#[derive(Logos, Debug, PartialEq)]
enum Token {
    // keyword
    #[token("fn")]
    Fn,
    
    #[token("let")]
    Let,

    #[token("struct")]
    Struct,

    #[token("trait")]
    Trait,

    // identifier
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    // number
    #[regex("[0-9]+")]
    Number,

    // operator
    #[token("=")]
    Assign,
    
    #[token(";")]
    Semicolon,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    // paren
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    // psace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}


pub fn count_lines_in_directory(dir: &Path) -> (usize, usize) {
    let mut total_lines = 0;
    let mut test_lines = 0;
    for entry in fs::read_dir(dir).expect(
        format!("Directory {} not found", dir.to_str().unwrap()).as_str()) {
        let e = entry.expect("Error reading entry");
        let path = e.path();
        if path.is_dir() {
            let (sub_total, sub_test) = count_lines_in_directory(&path);
            total_lines += sub_total;
            test_lines += sub_test;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let (file_total, file_test) = count_lines_in_file(&path);
            total_lines += file_total;
            test_lines += file_test;
        }
    }
    (total_lines, test_lines)
}


fn count_lines_in_file(path:&Path) -> (usize, usize) {
    let mut total_lines = 0;
    let mut test_lines = 0;

    let mut counting_testing = false;
    let content = fs::read_to_string(path).expect("Error reading file");
    let mut brace:Option<BraceCnt> = None;
    for _line in content.lines() {
        let line = _line.trim();
        if line.is_empty() {
            continue;
        }

        total_lines += 1;

        if line.starts_with("#[cfg(test)]") {
            if !counting_testing {
                counting_testing = true
            }
        } else if line.starts_with("#[test]") {
            if !counting_testing {
                counting_testing = true;
            }
        }

        if  counting_testing {
            test_lines += 1;
            let end = match &mut brace {
                Some(b) => {
                    b.lex_line(line);
                    b.lex_end()
                }
                None => {
                    let mut b = BraceCnt::new();
                    b.lex_line(line);
                    let end = b.lex_end();
                    brace = Some(b);
                    end
                }
            };
            if end {
                brace = None;
                counting_testing = false;
            }
        }
    }
    (total_lines, test_lines)
}


struct BraceCnt {
    found_brace: bool,
    brace_cnt : usize,
}

impl BraceCnt {
    fn new() -> Self {
        Self {
            found_brace : false,
            brace_cnt : 0
        }
    }

    fn lex_end(&self) -> bool {
        self.brace_cnt == 0 && self.found_brace
    }
    
    fn lex_line(&mut self, text:&str) {
        let lexer = Token::lexer(text);
        for token in lexer {
            if token == Ok(Token::LBrace) {
                self.brace_cnt += 1;
                if self.brace_cnt == 1 && !self.found_brace {
                    self.found_brace = true;
                }
            } else if token == Ok(Token::RBrace) {
                self.brace_cnt -= 1;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use crate::sloc::Token;
    use logos::Logos;

    #[test]
    fn test() {
        let source = r#"
            fn main() {
                let x = 42;
                let y = x + 1;
            }
        "#;

        let lexer = Token::lexer(source);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}
