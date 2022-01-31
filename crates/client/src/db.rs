use super::*;
use std::collections::HashMap;

/// Alias of `Ring<ChannelEvent>`.
pub type Events = rat::ring::Ring<ChannelEvent>;

/// Alias of `HashMap<RoomId, Events>`.
pub type Rooms = HashMap<RoomId, (Room, Events)>;

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
            self.rooms
                .entry(room.id)
                .or_insert((room, Default::default()))
                .1
                .push(event);
        }
    }
}

pub mod fake {
    use super::*;

    pub fn db() -> Db {
        let romain = user("Romain");
        let john = user("John");
        let mike = user("Mike");
        let sarah = user("Sarah");
        let room1 = room("Room 1");
        let room2 = room("Room 2");
        let room3 = room("Room 3");

        let users = &[&romain, &john, &mike, &sarah];
        let rooms = &[&room1, &room2, &room2];

        let mut db = Db::new(romain.clone());

        for user in users {
            db.push(enter(user, None));
            db.push(post(user, None, Some("Hello!")));
            db.push(post(user, None, None));

            for room in rooms {
                db.push(enter(user, Some(room)));
                db.push(post(user, Some(room), Some("Hello!")));
            }
        }

        for _ in 0..0 {
            db.push(post(&romain, None, None));
            db.push(post(&sarah, None, None));
            db.push(post(&john, None, None));
            db.push(post(&mike, None, None));
        }

        db
    }

    fn user(name: &str) -> User {
        User {
            id:   UserId::new(),
            name: name.into(),
        }
    }

    fn room(name: &str) -> Room {
        Room {
            id:   RoomId::new(),
            name: name.into(),
        }
    }

    fn message(body: Option<&str>) -> Message {
        Message {
            body: body.map(Into::into).unwrap_or_else(lorem),
        }
    }

    fn lorem() -> String {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into()
    }

    fn enter(user: &User, room: Option<&Room>) -> Event<User, Room> {
        Event::new(channel(room), user.clone(), EventType::Enter)
    }

    fn leave(user: &User, room: Option<&Room>) -> Event<User, Room> {
        Event::new(channel(room), user.clone(), EventType::Leave)
    }

    fn post(user: &User, room: Option<&Room>, msg: Option<&str>) -> Event<User, Room> {
        Event::new(channel(room), user.clone(), EventType::Post {
            message: message(msg),
        })
    }

    fn channel<T: Clone>(room: Option<&T>) -> Channel<T> {
        if let Some(room) = room {
            Channel::Room(room.clone())
        } else {
            Channel::World
        }
    }
}
