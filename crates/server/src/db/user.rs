use super::*;

/// A user.
#[derive(Clone, Debug)]
pub struct User {
    pub id:        UserId,
    pub client_id: ClientId,
    pub name:      String,
    pub room_ids:  Vec<RoomId>,
}

impl User {
    pub fn new(client_id: ClientId, name: String) -> Self {
        Self {
            id: UserId::new(),
            client_id,
            name,
            room_ids: Vec::new(),
        }
    }
}
