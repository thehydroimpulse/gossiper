
#[deriving(Show, Clone, Eq, PartialEq, Hash)]
pub struct Addr {
    pub ip: String,
    pub port: u16
}

impl Addr {
    pub fn new(ip: &str, port: u16) -> Addr {
        Addr {
            ip: ip.to_string(),
            port: port
        }
    }
}
