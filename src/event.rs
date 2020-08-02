use crate::object;
use std::collections::HashMap;
#[derive(Debug)]
pub struct EventX {
    event: String,
    data: String,
}

pub type EventFunction = fn(EventX);

#[derive(Debug)]
pub struct EventHandler {
    handler: EventFunction,
    thisobj: usize,
}

#[derive(Debug)]
pub struct MiniDispatcher {
    event_listeners: HashMap<String, Vec<EventHandler>>,
}

fn find(_vec: &mut Vec<EventHandler>, _handler: &mut EventHandler) {
    for value in _vec.iter_mut() {
        // assert!(value.thisobj == _handler.thisobj);
        // let b = object::is_same(&value.thisobj, &_handler.thisobj);
        // println!("{}", b);
        if value.handler == _handler.handler {}
    }
}

impl MiniDispatcher {
    pub fn new() -> Self {
        Self {
            event_listeners: HashMap::new(),
        }
    }

    pub fn on(&mut self, event: String, handler: EventFunction, thisobj: usize) {
        if false == self.event_listeners.contains_key(&event) {
            self.event_listeners.insert(event.clone(), Vec::new());
        }

        // println!("{:?}",thisobj.unwrap());

        let vec = self.event_listeners.get_mut(&event).unwrap();
        let mut d = EventHandler { handler, thisobj };

        find(vec, &mut d);

        vec.push(d);

        // println!("{:?}", vec);

        // let mut t:Vec<EventHandler> = Vec::new();
        // t.push(d);
    }

    // pub fn _off(_event: String, _handler: EventFunction, _thisobj: MiniDispatcher) {}

    // pub fn _displatch(_event: EventX) {}
}

// static ref default_dispatcher = MiniDispather::new();

// lazy_static! {
//     pub static ref facade: MiniDispatcher = {
//         let d = MiniDispatcher::new();
//         d
//     };
// }

// pub static mut facade: MiniDispatcher = MiniDispatcher::new();
