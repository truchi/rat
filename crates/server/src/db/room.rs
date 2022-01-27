use super::*;

/// A room.
#[derive(Clone, Debug)]
pub struct Room {
    pub id:       RoomId,
    pub user_ids: Vec<UserId>,
}

impl Room {
    pub fn user_ids(&self) -> std::slice::Iter<UserId> {
        self.user_ids.iter()
    }
}
