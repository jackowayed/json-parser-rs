pub mod lexer {
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

    pub fn lex(input: String) -> Vec<Token> {
        let mut tokens = vec![];
        let mut char_iter = input.chars();
        while let Some(c) = char_iter.next() {
            let new_token = match c {
                '0'..='9' | '-' => Some(number(&mut char_iter, c)),
                '\t' | ' ' | '\n' | '\r' => None, // skip whitespace
                //'"' => self.string(it),
                ':' | ',' | '[' | ']' | '{' | '}' => Some(symbol(c)),
                't' | 'f' | 'n' => Some(alpha_literal(&mut char_iter, c)),
                _ => todo!("more matches coming"),
            };
            if let Some(token) = new_token {
                tokens.push(token);
            }
        }
        tokens
    }

    fn alpha_literal(char_iter: &mut std::str::Chars, first_char: char) -> Token {
        use Token::*;
        match first_char {
            't' => {
                expect(char_iter, "rue");
                Boolean(true)
            }
            'f' => {
                expect(char_iter, "alse");
                Boolean(false)
            }
            _ => todo!(),
        }
    }

    fn expect(char_iter: &mut std::str::Chars, next_chars: &str) {
        let token: String = char_iter.take(next_chars.len()).collect();
        assert_eq!(token, next_chars); // == next_chars;
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
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use lexer::Token::*;
    // https://stackoverflow.com/a/38183903
    #[allow(unused_macros)]
    macro_rules! vec_of_strings {
      ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn number() {
        assert_eq!(
            vec![Number("-37".to_string())],
            lexer::lex("  -37 ".to_string())
        );
    }

    #[test]
    fn bool() {
        assert_eq!(vec![Boolean(false)], lexer::lex("\n\tfalse ".to_string()));
    }

    #[test]
    #[should_panic]
    fn bad_literal() {
        lexer::lex("fxx".to_string());
    }
}
