use uuid::Uuid;
use server::Server;

#[deriving(Encodable, Decodable, PartialEq, Show)]
pub struct Join {
    id: Uuid,
    ip: String,
    port: u16
}

impl Join {
    pub fn new<'a, T>(server: &Server<'a, T>) -> Join {
        Join {
            id: server.id,
            ip: server.ip.to_string(),
            port: server.port
        }
    }
}
