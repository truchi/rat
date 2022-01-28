//! [`Db`], [`Client`], [`User`] and [`Room`].

use super::*;

mod client;
mod room;
mod user;

pub use client::*;
pub use room::*;
pub use user::*;

use std::ops::Index;
use std::ops::IndexMut;

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

    pub fn find<T: Iter, F: FnMut(&T) -> bool>(&self, mut predicate: F) -> Option<&T> {
        self.iter::<T>()
            .map(|(_, value)| value)
            .find(|value| predicate(value))
    }

    pub fn channel<T>(&self, event: &Event<T>) -> impl Iterator<Item = &Client> {
        match event.channel {
            Channel::World => Box::new(self.world()) as Box<dyn Iterator<Item = _>>,
            Channel::Room { room_id } => Box::new(self.room(room_id)),
        }
    }

    pub fn world(&self) -> impl Iterator<Item = &Client> {
        self.iter::<User>()
            .map(|(&user_id, _)| self.client(user_id))
    }

    pub fn room(&self, room_id: RoomId) -> impl Iterator<Item = &Client> {
        self[room_id]
            .user_ids()
            .map(|&user_id| self.client(user_id))
    }

    pub fn is_in(&self, user_id: UserId, room_id: RoomId) -> Result<bool, ()> {
        let user = self.get(&user_id).ok_or(())?;
        let room = self.get(&room_id).ok_or(())?;

        if user.is_in(room_id) {
            debug_assert!(room.has(user_id));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn enter(&mut self, user_id: UserId, room_id: RoomId) -> Result<(), ()> {
        self.get(&room_id).ok_or(())?;
        self.get_mut(&user_id).ok_or(())?.enter(room_id);
        self.get_mut(&room_id).unwrap().enter(user_id);

        Ok(())
    }

    pub fn leave(&mut self, user_id: UserId, room_id: RoomId) -> Result<(), ()> {
        self.get(&room_id).ok_or(())?;
        self.get_mut(&user_id).ok_or(())?.leave(room_id);
        self.get_mut(&room_id).unwrap().leave(user_id);

        Ok(())
    }
}

impl Db {
    fn client(&self, user_id: UserId) -> &Client {
        let client_id = self[user_id].client_id;

        &self[client_id]
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

        impl Index<$K> for Db {
            type Output = $V;

            fn index(&self, index: $K) -> &Self::Output {
                self.get(&index).unwrap_or_else(|| panic!("{} {:?} not found", stringify!($Type), index))
            }
        }

        impl IndexMut<$K> for Db {
            fn index_mut(&mut self, index: $K) -> &mut Self::Output {
                self.get_mut(&index).unwrap_or_else(|| panic!("{} {:?} not found", stringify!($Type), index))
            }
        }
    )* };
}

impls!(
    (ClientId, Client), clients;
    (UserId, User), users;
    (RoomId, Room), rooms;
);
