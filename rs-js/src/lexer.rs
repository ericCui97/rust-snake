pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}


#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(ch) = self.current_char {
            // Implement tokenization logic here
            // Example: Identifiers
            if ch.is_alphabetic() {
                let mut identifier = String::new();
                while let Some(ch) = self.current_char {
                    if ch.is_alphanumeric() {
                        identifier.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                return Some(Token::Identifier(identifier));
            }

            // Example: Numbers
            if ch.is_digit(10) {
                let mut number = String::new();
                while let Some(ch) = self.current_char {
                    if ch.is_digit(10) {
                        number.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                return Some(Token::Number(number.parse().unwrap()));
            }

            // Example: Operators
            if "+-*/".contains(ch) {
                self.advance();
                return Some(Token::Operator(ch));
            }
        }

        None
    }
}
#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Operator(char),
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_next_token_identifier() {
        let input = String::from("abc");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Identifier(String::from("abc"))));
    }

    #[test]
    fn test_next_token_number() {
        let input = String::from("123");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Number(123)));
    }

    #[test]
    fn test_next_token_operator() {
        let input = String::from("+");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Operator('+')));
    }
}


#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_next_token_identifier() {
        let input = String::from("abc");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Identifier(String::from("abc"))));
    }

    #[test]
    fn test_next_token_number() {
        let input = String::from("123");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Number(123)));
    }

    #[test]
    fn test_next_token_operator() {
        let input = String::from("+");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Operator('+')));
    }

    #[test]
    fn test_next_token_whitespace() {
        let input = String::from("   abc   ");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Identifier(String::from("abc"))));
    }

    #[test]
    fn test_next_token_empty() {
        let input = String::from("");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), None);
    }
}
