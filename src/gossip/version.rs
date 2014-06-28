use serialize::{Encodable};

#[deriving(Show, PartialEq, Encodable, Decodable)]
pub struct Version(pub u8);
