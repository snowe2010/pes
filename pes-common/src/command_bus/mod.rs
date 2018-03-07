// std
use std::collections::HashMap;
use std::marker::PhantomData;

// uuid
use uuid::Uuid;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

// Note: This doesn't support Copy or Clone for safety reasons.
// More specifically, it should be impossible to unregister the same handler more than once.
pub struct CommandHandlerId<T: Command + ?Sized> {
    id: Uuid,
    _t: PhantomData<T>,
}
impl<T: Command + ?Sized> Eq for CommandHandlerId<T> {}
impl<T: Command + ?Sized> PartialEq for CommandHandlerId<T> {
    fn eq(&self, other: &Self) -> bool {
        // PhantomData is only included for completeness.
        self.id == other.id && self._t == other._t
    }
}

struct CommandHandler<T: Command + ?Sized> {
    priority: i32,
    f: fn(&mut T), // Multiple handlers for the same event/bus.
    id: CommandHandlerId<T>,
}

pub struct CommandMetadata<T: Command + ?Sized> {
    // This should be changed to allow for non-static CommandBus.
    handlers: HashMap<&'static CommandBus, Vec<CommandHandler<T>>>,
}

impl<T: Command + ?Sized> CommandMetadata<T> {
    pub fn new() -> CommandMetadata<T> {
        CommandMetadata { handlers: HashMap::new() }
    }

    fn put(&mut self, bus: &'static CommandBus, f: fn(&mut T), priority: i32) -> CommandHandlerId<T> {
        // Sorted based on priority
        let vec = self.handlers.entry(bus).or_insert_with(Vec::new);
        let pos = vec.binary_search_by(|a| a.priority.cmp(&priority)).unwrap_or_else(|e| e);
        let id = Uuid::new_v4();
        vec.insert(pos, CommandHandler { f, priority, id: CommandHandlerId { id, _t: PhantomData } });
        CommandHandlerId { id, _t: PhantomData } // Single-use removal key
    }

    fn remove(&mut self, bus: &CommandBus, f: CommandHandlerId<T>) {
        let flag = self.handlers.get_mut(bus).iter_mut().any(|v| { v.retain(|x| x.id != f); v.is_empty() });
        if flag { self.handlers.remove(bus); }
    }

    fn post(&self, bus: &CommandBus, event: &mut T) -> bool {
        self.handlers.get(bus).iter().flat_map(|x| x.iter()).any(|h| {
            (h.f)(event);
            event.cancelled()
        })
    }
}

pub trait Command {
    // type properties
    fn event_metadata<F, R>(F) -> R where F: FnOnce(&CommandMetadata<Self>) -> R;

    fn mut_metadata<F, R>(F) -> R where F: FnOnce(&mut CommandMetadata<Self>) -> R;

    fn cancellable() -> bool { false }

    // instance properties
    fn cancelled(&self) -> bool { false }

    fn cancel(&mut self, bool) { panic!() }
}

#[derive(PartialEq, Eq, Hash)]
pub struct CommandBus {
    uuid: Uuid
}

impl CommandBus {
    pub fn new() -> CommandBus {
        CommandBus { uuid: Uuid::new_v4() }
    }

    pub fn register<T>(&'static self, f: fn(&mut T), priority: i32) -> CommandHandlerId<T> where T: Command {
        T::mut_metadata(|x| x.put(self, f, priority))
    }

    pub fn unregister<T>(&self, f: CommandHandlerId<T>) where T: Command {
        T::mut_metadata(|x| x.remove(self, f))
    }

    pub fn post<T>(&self, event: &mut T) -> bool where T: Command {
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
