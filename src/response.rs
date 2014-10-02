use uuid::Uuid;

pub struct Response {
    broadcast_id: Uuid,
    bytes: Vec<u8>
}

impl Response {
    pub fn new(id: Uuid) -> Response {
        Response {
            broadcast_id: id,
            bytes: Vec::new()
        }
    }
}
