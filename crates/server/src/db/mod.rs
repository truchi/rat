//! [`Db`], [`Client`], [`User`] and [`Room`].

use super::*;

mod client;
mod room;
mod user;

pub use client::*;
pub use room::*;
pub use user::*;

#[doc(hidden)]
macro_rules! hashmap_aliases {
    ($($Type:ident = <$K:ident, $V:ident>)*) => { $(
        #[doc = concat!("Alias of `HashMap<`[`", stringify!($K), "`]`, `[`", stringify!($V), "`]`>`.")]
        pub type $Type = HashMap<$K, $V>;
    )* };
}

hashmap_aliases!(
    Clients = <ClientId, Client>
    Users = <UserId, User>
    Rooms = <RoomId, Room>
);

/// The db.
#[derive(Debug)]
pub struct Db {
    clients: Clients,
    users:   Users,
    rooms:   Rooms,
}

impl Db {
    pub fn new() -> Self {
        Self {
            clients: Clients::default(),
            users:   Users::default(),
            rooms:   Rooms::default(),
        }
    }

    pub fn insert<T: Insert>(&mut self, insert: T) -> Option<T::V> {
        T::insert(insert, self)
    }

    pub fn get<T: Get>(&self, key: T) -> Option<&T::V> {
        key.get(self)
    }

    pub fn get_mut<T: Get>(&mut self, key: T) -> Option<&mut T::V> {
        key.get_mut(self)
    }

    pub fn iter<T: Iter>(&self) -> std::collections::hash_map::Iter<T::K, T> {
        T::iter(self)
    }

    pub fn channel(&self, event: &Event) -> impl Iterator<Item = &Client> {
        match event.channel {
            Channel::World => Box::new(self.world()) as Box<dyn Iterator<Item = _>>,
            Channel::Room { room_id } => Box::new(self.room(room_id)),
            Channel::Private { user_id } => Box::new(self.private([user_id, event.user_id])),
        }
    }

    pub fn world(&self) -> impl Iterator<Item = &Client> {
        self.iter::<User>()
            .map(|(&user_id, _)| self.client(user_id))
    }

    pub fn room(&self, room_id: RoomId) -> impl Iterator<Item = &Client> {
        self.get(&room_id)
            .expect("Room not found")
            .user_ids()
            .map(|&user_id| self.client(user_id))
    }

    pub fn private(&self, user_ids: [UserId; 2]) -> impl Iterator<Item = &Client> {
        user_ids.into_iter().map(|user_id| self.client(user_id))
    }
}

impl Db {
    fn client(&self, user_id: UserId) -> &Client {
        self.get(&self.get(&user_id).expect("User not found").client_id)
            .expect("Client not found")
    }
}

/// [`insert`](Insert::insert) for [`Db`].
pub trait Insert {
    type V;

    fn insert(self, db: &mut Db) -> Option<Self::V>;
}

/// [`get`](Get::get)/[`get_mut`](Get::get_mut) for [`Db`].
pub trait Get {
    type V;

    fn get(self, db: &Db) -> Option<&Self::V>;
    fn get_mut(self, db: &mut Db) -> Option<&mut Self::V>;
}

/// [`iter`](Iter::iter) for [`Db`].
pub trait Iter: Sized {
    type K;

    fn iter(db: &Db) -> std::collections::hash_map::Iter<Self::K, Self>;
}

#[doc(hidden)]
macro_rules! impls {
    ($(($K:ty, $V:ty), $field:ident;)*) => { $(
        impl Insert for ($K, $V) {
            type V = $V;

            fn insert(self, db: &mut Db) -> Option<Self::V> {
                db.$field.insert(self.0, self.1)
            }
        }

        impl Get for &$K {
            type V = $V;

            fn get(self, db: &Db) -> Option<&Self::V> {
                db.$field.get(self)
            }

            fn get_mut(self, db: &mut Db) -> Option<&mut Self::V> {
                db.$field.get_mut(self)
            }
        }

        impl Iter for $V {
            type K = $K;

            fn iter(db: &Db) -> std::collections::hash_map::Iter<Self::K, Self> {
                (&db.$field).into_iter()
            }
        }
    )* };
}

impls!(
    (ClientId, Client), clients;
    (UserId, User), users;
    (RoomId, Room), rooms;
);