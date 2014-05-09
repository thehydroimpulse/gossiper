use std::str::Chars;

pub enum Protocol {
    Version(uint),
    Binary(Vec<u8>),
    Text(~str)
}

pub enum TextProtocol {
    Key(~str),
    Value(Vec<u8>)
}

// The TextProtocol's parser. All we need to spit out is a key->value combination.
// Because we allow streaming within the protocol, we'll only return each
// key->value pair once we have fully parsed both.
//
// There could be an option to receive the raw data as we get it for the value,
// but not the key.
//
// ```rust
// Parser::new()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_parser() {
        let mut parser = Parser::new("hello world");
        assert_eq!(parser.iter.next().unwrap(), 'h');
    }
}