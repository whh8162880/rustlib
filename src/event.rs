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
    thisobj: *mut usize,
}

#[derive(Debug)]
pub struct MiniDispatcher {
    event_listeners: HashMap<String, Vec<EventHandler>>,
}

#[allow(dead_code)] 
fn find(_vec: &mut Vec<EventHandler>, _handler: &mut EventHandler) -> i16 {
    let mut index = 0;

    for value in _vec.iter_mut() {
        if value.thisobj == _handler.thisobj && value.handler == _handler.handler {
            break;
        }
        index += 1;
        // let b = object::is_same(&value.thisobj, &_handler.thisobj);
    }

    if index == _vec.len() as i16 {
        index = -1;
    }

    return index;
}

impl MiniDispatcher {
    #[allow(dead_code)] 
    pub fn new() -> Self {
        Self {
            event_listeners: HashMap::new(),
        }
    }

    #[allow(dead_code)] 
    pub fn on(&mut self, event: String, handler: EventFunction, thisobj: *mut usize) {
        if false == self.event_listeners.contains_key(&event) {
            self.event_listeners.insert(event.clone(), Vec::new());
        }

        // println!("{:?}",thisobj.unwrap());

        let vec = self.event_listeners.get_mut(&event).unwrap();
        let mut d = EventHandler { handler, thisobj };

        let i = find(vec, &mut d);
        println!("{}", i);

        if i == -1 {
            vec.push(d);
        }

        // println!("{:?}", vec);

        // let mut t:Vec<EventHandler> = Vec::new();
        // t.push(d);
    }

    pub fn _off(_event: String, _handler: EventFunction, _thisobj: *mut usize) {}

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
