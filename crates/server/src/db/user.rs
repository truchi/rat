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

    pub fn room_ids(&self) -> std::slice::Iter<RoomId> {
        self.room_ids.iter()
    }
}

impl Into<rat::User> for User {
    fn into(self) -> rat::User {
        rat::User {
            id:   self.id,
            name: self.name,
        }
    }
}
