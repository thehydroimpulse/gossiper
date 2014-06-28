use serialize::{Encodable, Decodable};

#[deriving(Show, PartialEq, Decodable, Encodable)]
pub struct Version(u8);
