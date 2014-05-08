pub enum Protocol {
    Version(uint),
    Binary(Vec<u8>),
    Text(~str)
}