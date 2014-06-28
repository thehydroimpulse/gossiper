use serialize::{Encodable};

#[deriving(Show, PartialEq, Encodable, Decodable)]
pub struct Version(u8);
