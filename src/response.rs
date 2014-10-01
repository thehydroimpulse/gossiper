use uuid::Uuid;

pub struct Response {
    broadcast_id: Uuid,
    bytes: Vec<u8>
}
