use std::str::{Chars, from_char};

pub enum Protocol {
    Version(uint),
    Binary(Vec<u8>),
    Text(~str)
}

pub enum TextProtocol {
    Key(~str),
    Value(Vec<u8>)
}

#[deriving(Eq,Show)]
pub enum TextAst {
    Empty,
    KeyVal(~str, ~str)
}

// The TextProtocol's parser. All we need to spit out is a key->value combination.
// Because we allow streaming within the protocol, we'll only return each
// key->value pair once we have fully parsed both.
//
// There could be an option to receive the raw data as we get it for the value,
// but not the key.
//
// ```rust
// Parser::new("hello world")
// ```
pub struct Parser<'a> {
    input: &'a str,
    iter: Chars<'a>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            input: input,
            iter: input.chars()
        }
    }

    pub fn parse(&mut self) -> TextAst {

        if self.input.len() == 0 {
            return Empty;
        }

        let mut key = "".to_owned();
        let mut value = "".to_owned();
        let mut c = self.iter.next().unwrap();

        if c.is_alphanumeric() {
            key = key.append(from_char(c));
            while c != '[' {
                c = self.iter.next().unwrap();

                if c != '[' {
                    key = key.append(from_char(c));
                }
            }

            // Start the value
            if c == '[' {
                while c != ']' {
                    c = self.iter.next().unwrap();

                    if c != ']' {
                        value = value.append(from_char(c));
                    }
                }
            }
        }

        KeyVal(key, value)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_parser() {
        let mut parser = Parser::new("hello world");
        assert_eq!(parser.iter.next().unwrap(), 'h');
    }

    #[test]
    fn parser_parse_empty_string() {
        let mut parser = Parser::new("");
        assert_eq!(parser.parse(), Empty);
    }

    #[test]
    fn parse_simple_kv() {
        let mut parser = Parser::new("Version[1]");
        assert_eq!(parser.parse(), KeyVal("Version".to_owned(), "1".to_owned()));
    }
}