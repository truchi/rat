use super::*;
use std::collections::HashMap;

/// Alias of `Ring<ChannelEvent>`.
pub type Events = rat::ring::Ring<ChannelEvent>;

/// Alias of `HashMap<RoomId, Events>`.
pub type Rooms = HashMap<RoomId, Events>;

/// A channel event.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ChannelEvent {
    Enter { user: User },
    Leave { user: User },
    Post { user: User, message: Message },
}

impl ChannelEvent {
    pub fn is_leaving(&self, user: &User) -> bool {
        if let ChannelEvent::Leave { user: leaving } = self {
            if leaving == user {
                return true;
            }
        }

        false
    }

    pub fn from(event: Event<User, Room>) -> (Channel<Room>, Self) {
        let channel = event.channel;
        let user = event.user;
        let event = match event.event_type {
            EventType::Enter => ChannelEvent::Enter { user },
            EventType::Leave => ChannelEvent::Leave { user },
            EventType::Post { message } => ChannelEvent::Post { user, message },
        };

        (channel, event)
    }
}

#[derive(Clone, Debug)]
pub struct Db {
    user:  User,
    world: Events,
    rooms: Rooms,
}

impl Db {
    pub fn new(user: User) -> Self {
        Self {
            user,
            world: Default::default(),
            rooms: Default::default(),
        }
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    pub fn world(&self) -> &Events {
        &self.world
    }

    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }

    pub fn push(&mut self, event: Event<User, Room>) {
        match ChannelEvent::from(event) {
            (Channel::World, event) => self.push_world(event),
            (Channel::Room(room), event) => self.push_room(room, event),
        }
    }

    fn push_world(&mut self, event: ChannelEvent) {
        if event.is_leaving(&self.user) {
            self.world.clear();
        }

        self.world.push(event);
    }

    fn push_room(&mut self, room: Room, event: ChannelEvent) {
        if event.is_leaving(&self.user) {
            self.rooms.remove(&room.id);
        } else {
            self.rooms.entry(room.id).or_default().push(event);
        }
    }
}
