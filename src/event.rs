use super::*;

impl UserId {
    pub fn enter(self, channel: Channel) -> Event<UserId> {
        Event::new(channel, self, EventType::Enter)
    }

    pub fn leave(self, channel: Channel) -> Event<UserId> {
        Event::new(channel, self, EventType::Leave)
    }

    pub fn post(self, channel: Channel, message: Message) -> Event<UserId> {
        Event::new(channel, self, EventType::Post { message })
    }

    pub fn enter_world(self) -> Event<UserId> {
        self.enter(Channel::World)
    }

    pub fn leave_world(self) -> Event<UserId> {
        self.leave(Channel::World)
    }

    pub fn post_world(self, message: Message) -> Event<UserId> {
        self.post(Channel::World, message)
    }

    pub fn enter_room(self, room_id: RoomId) -> Event<UserId> {
        self.enter(Channel::Room { room_id })
    }

    pub fn leave_room(self, room_id: RoomId) -> Event<UserId> {
        self.leave(Channel::Room { room_id })
    }

    pub fn post_room(self, room_id: RoomId, message: Message) -> Event<UserId> {
        self.post(Channel::Room { room_id }, message)
    }
}
