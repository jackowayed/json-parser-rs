#[derive(PartialEq, Debug)]
pub enum Token {
    Number(String),
    String(String),
    Boolean(bool),
    Null,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut char_iter = input.chars();
    while let Some(c) = char_iter.next() {
        tokens.push(match c {
            '0'..='9' | '-' => number(&mut char_iter, c),
            '\t' | ' ' | '\n' | '\r' => continue, // skip whitespace
            '"' => string(&mut char_iter)?,
            ':' | ',' | '[' | ']' | '{' | '}' => symbol(c),
            't' | 'f' | 'n' => alpha_literal(&mut char_iter, c)?,
            _ => return Err("invalid syntax".to_string()),
        });
    }
    Ok(tokens)
}

fn string(char_iter: &mut std::str::Chars) -> TokenResult {
    let mut str = String::new();
    let mut prior_was_backslash = false;
    while let Some(c) = char_iter.next() {
        if prior_was_backslash {
            str.push(c);
            prior_was_backslash = false;
            continue;
        }
        match c {
            '\\' => {
                prior_was_backslash = true;
            }
            '"' => {
                return Ok(Token::String(str));
            }
            _ => str.push(c),
        }
    }
    return Err("unterminated string".to_string());
}

type TokenResult = Result<Token, String>;

fn alpha_literal(char_iter: &mut std::str::Chars, first_char: char) -> TokenResult {
    use Token::*;
    Ok(match first_char {
        't' => {
            expect(char_iter, "rue")?;
            Boolean(true)
        }
        'f' => {
            expect(char_iter, "alse")?;
            Boolean(false)
        }
        'n' => {
            expect(char_iter, "ull")?;
            Null
        }
        _ => todo!(),
    })
}

fn expect(char_iter: &mut std::str::Chars, next_chars: &str) -> Result<(), String> {
    let token: String = char_iter.take(next_chars.len()).collect();
    return if token != next_chars {
        Err("unknown identifier".to_string())
    } else {
        Ok(())
    };
}

fn symbol(c: char) -> Token {
    use Token::*;
    match c {
        ':' => Colon,
        ',' => Comma,
        '[' => LeftBracket,
        ']' => RightBracket,
        '{' => LeftBrace,
        '}' => RightBrace,
        _ => unreachable!("only call this method for the above"),
    }
}

fn number(char_iter: &mut std::str::Chars, first_char: char) -> Token {
    let mut num_str = first_char.to_string();
    while let Some(c) = char_iter.next() {
        match c {
            '0'..='9' => num_str.push(c),
            // TODO decimel points, hex, etc.
            _ => break,
        }
    }
    Token::Number(num_str)
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use Token::*;
    // https://stackoverflow.com/a/38183903
    #[allow(unused_macros)]
    macro_rules! vec_of_strings {
      ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn number() {
        assert_eq!(
            Ok(vec![Number("-37".to_string())]),
            lex("  -37 ".to_string())
        );
    }

    #[test]
    fn bool() {
        assert_eq!(Ok(vec![Boolean(false)]), lex("\n\tfalse ".to_string()));
    }

    #[test]
    fn bad_literal() {
        assert!(lex("fxx".to_string()).is_err());
        assert!(lex("x12".to_string()).is_err());
    }

    #[test]
    fn symbols() {
        assert_eq!(Ok(vec![Colon]), lex(" : ".to_string()));
        assert!(lex(">".to_string()).is_err());
    }

    #[test]
    fn string() {
        assert_eq!(
            Ok(vec![Token::String("foo bar".to_string())]),
            lex(" \"foo bar\"  ".to_string())
        );
        assert!(lex("\"foo \\\" bar".to_string()).is_err());
    }

    #[test]
    fn combo() {
        assert_eq!(
            Ok(vec![
                LeftBrace,
                Token::String("foo".to_string()),
                Colon,
                Null,
                RightBrace
            ]),
            lex("{  \"foo\": null} ".to_string())
        );
    }
}
