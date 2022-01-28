use super::*;

/// A room.
#[derive(Clone, Debug)]
pub struct Room {
    pub id:       RoomId,
    pub name:     String,
    pub user_ids: Vec<UserId>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            id: RoomId::new(),
            name,
            user_ids: Vec::new(),
        }
    }

    pub fn user_ids(&self) -> std::slice::Iter<UserId> {
        self.user_ids.iter()
    }

    pub fn has(&self, user_id: UserId) -> bool {
        self.user_ids().find(|&&id| id == user_id).is_some()
    }

    pub fn enter(&mut self, user_id: UserId) {
        if !self.has(user_id) {
            self.user_ids.push(user_id);
        }
    }

    pub fn leave(&mut self, user_id: UserId) {
        if let Some(index) = self.user_ids().position(|&id| id == user_id) {
            self.user_ids.swap_remove(index);
        }
    }
}

impl Into<rat::Room> for Room {
    fn into(self) -> rat::Room {
        rat::Room {
            id:   self.id,
            name: self.name,
        }
    }
}
