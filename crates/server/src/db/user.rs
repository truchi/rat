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

    pub fn is_in(&self, room_id: RoomId) -> bool {
        self.room_ids().find(|&&id| id == room_id).is_some()
    }

    pub fn enter(&mut self, room_id: RoomId) {
        if !self.is_in(room_id) {
            self.room_ids.push(room_id);
        }
    }

    pub fn leave(&mut self, room_id: RoomId) {
        if let Some(index) = self.room_ids().position(|&id| id == room_id) {
            self.room_ids.swap_remove(index);
        }
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
