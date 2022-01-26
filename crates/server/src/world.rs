use super::*;

pub type Clients = HashMap<Uuid, ClientData>;
pub type Users = HashMap<User, UserData>;
pub type Rooms = HashMap<Room, Vec<User>>;

#[derive(Debug)]
pub struct ClientData {
    pub id:        Uuid,
    pub user:      Option<User>,
    pub to_client: ToClient,
}

impl ClientData {
    pub fn new(to_client: ToClient) -> Self {
        Self {
            id: Uuid::new_v4(),
            user: None,
            to_client,
        }
    }

    pub async fn send(&mut self, message: ManagerToClient) -> Result<(), ()> {
        self.to_client.send(message).await.map_err(|_| ())
    }

    pub async fn accepted(&mut self, id: Uuid) -> Result<(), ()> {
        self.send(Accepted(id)).await
    }

    pub async fn respond(&mut self, response: ServerResponse) -> Result<(), ()> {
        self.send(Response(response)).await
    }

    pub async fn event(&mut self, event: Event) -> Result<(), ()> {
        self.respond(Evented(event)).await
    }
}

#[derive(Debug)]
pub struct UserData {
    pub client_id: Uuid,
    pub name:      String,
    pub rooms:     Vec<Room>,
    pub privates:  Vec<User>,
}

impl UserData {
    pub fn new(client_id: Uuid, name: String) -> Self {
        Self {
            client_id,
            name,
            rooms: Vec::new(),
            privates: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct World {
    clients: Clients,
    users:   Users,
    rooms:   Rooms,
}

impl World {
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
}

pub trait Insert {
    type V;

    fn insert(self, world: &mut World) -> Option<Self::V>;
}

pub trait Get {
    type V;

    fn get(self, world: &World) -> Option<&Self::V>;
    fn get_mut(self, world: &mut World) -> Option<&mut Self::V>;
}

#[doc(hidden)]
macro_rules! impls {
    ($(($K:ty, $V:ty), $field:ident;)*) => { $(
        impl Insert for ($K, $V) {
            type V = $V;

            fn insert(self, world: &mut World) -> Option<Self::V> {
                world.$field.insert(self.0, self.1)
            }
        }

        impl Get for &$K {
            type V = $V;

            fn get(self, world: &World) -> Option<&Self::V> {
                world.$field.get(self)
            }

            fn get_mut(self, world: &mut World) -> Option<&mut Self::V> {
                world.$field.get_mut(self)
            }
        }
    )* };
}

impls!(
    (Uuid, ClientData), clients;
    (User, UserData), users;
    (Room, Vec<User>), rooms;
);
