use super::*;

/// A room.
#[derive(Debug)]
pub struct Room {
    pub id:       RoomId,
    pub user_ids: Vec<UserId>,
}
