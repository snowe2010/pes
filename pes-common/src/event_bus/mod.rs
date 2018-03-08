// std
use std::collections::HashMap;
use std::marker::PhantomData;

use postgres::{Connection, TlsMode};
// uuid
use uuid::Uuid;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

// Note: This doesn't support Copy or Clone for safety reasons.
// More specifically, it should be impossible to unregister the same handler more than once.
pub struct EventHandlerId<T: Event + ?Sized> {
    id: Uuid,
    _t: PhantomData<T>,
}

impl<T: Event + ?Sized> Eq for EventHandlerId<T> {}
impl<T: Event + ?Sized> PartialEq for EventHandlerId<T> {
    fn eq(&self, other: &Self) -> bool {
        // PhantomData is only included for completeness.
        self.id == other.id && self._t == other._t
    }
}

struct EventHandler<T: Event + ?Sized> {
    priority: i32,
    f: fn(&mut T), // Multiple handlers for the same event/bus.
    id: EventHandlerId<T>,
}

pub struct EventMetadata<T: Event + ?Sized> {
    // This should be changed to allow for non-static EventBus.
    handlers: HashMap<&'static EventBus, Vec<EventHandler<T>>>,
}

impl<T: Event + ?Sized> EventMetadata<T> {
    pub fn new() -> EventMetadata<T> {
        EventMetadata { handlers: HashMap::new() }
    }

    fn put(&mut self, bus: &'static EventBus, f: fn(&mut T), priority: i32) -> EventHandlerId<T> {
        // Sorted based on priority
        let vec = self.handlers.entry(bus).or_insert_with(Vec::new);
        let pos = vec.binary_search_by(|a| a.priority.cmp(&priority)).unwrap_or_else(|e| e);
        let id = Uuid::new_v4();
        vec.insert(pos, EventHandler { f, priority, id: EventHandlerId { id, _t: PhantomData } });
        EventHandlerId { id, _t: PhantomData } // Single-use removal key
    }

    fn remove(&mut self, bus: &EventBus, f: EventHandlerId<T>) {
        let flag = self.handlers.get_mut(bus).iter_mut().any(|v| { v.retain(|x| x.id != f); v.is_empty() });
        if flag { self.handlers.remove(bus); }
    }

    fn post(&self, bus: &EventBus, event: &mut T) -> bool {
        self.handlers.get(bus).iter().flat_map(|x| x.iter()).any(|h| {
            (h.f)(event);
            event.cancelled()
        })
    }
}

pub trait Event {
    // type properties
    fn event_metadata<F, R>(F) -> R where F: FnOnce(&EventMetadata<Self>) -> R;

    fn mut_metadata<F, R>(F) -> R where F: FnOnce(&mut EventMetadata<Self>) -> R;

    fn cancellable() -> bool { false }

    // instance properties
    fn cancelled(&self) -> bool { false }

    fn cancel(&mut self, bool) { panic!() }
}

#[derive(PartialEq, Eq, Hash)]
pub struct EventBus {
    uuid: Uuid,
    conn: Connection
}

impl EventBus {
    pub fn new() -> EventBus {
        let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
//        conn.execute("CREATE TABLE person (
//                    id              SERIAL PRIMARY KEY,
//                    name            VARCHAR NOT NULL,
//                    data            BYTEA
//                  )", &[]).unwrap();
//        let me = Person {
//            id: 0,
//            name: "Steven".to_string(),
//            data: None,
//        };
//        conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
//                     &[&me.name, &me.data]).unwrap();
//        for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
//            let person = Person {
//                id: row.get(0),
//                name: row.get(1),
//                data: row.get(2),
//            };
//            println!("Found person {}", person.name);
//        }
        EventBus { uuid: Uuid::new_v4(), conn }
    }

    pub fn register<T>(&'static self, f: fn(&mut T), priority: i32) -> EventHandlerId<T> where T: Event {
        T::mut_metadata(|x| x.put(self, f, priority))
    }

    pub fn unregister<T>(&self, f: EventHandlerId<T>) where T: Event {
        T::mut_metadata(|x| x.remove(self, f))
    }

    pub fn post<T>(&self, event: &mut T) -> bool where T: Event {
        T::event_metadata(|x| x.post(self, event))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
