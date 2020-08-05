use std::collections::HashMap;
#[derive(Debug)]
pub struct EventX {
    pub event: String,
    pub data: *mut usize,
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
fn find(vec: &mut Vec<EventHandler>, handler: EventFunction, thisobj: *mut usize) -> i16 {
    let mut index = 0;

    for value in vec.iter_mut() {
        if value.thisobj == thisobj && value.handler == handler {
            break;
        }
        index += 1;
        // let b = object::is_same(&value.thisobj, &_handler.thisobj);
    }

    if index == vec.len() as i16 {
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
    pub fn on<T>(&mut self, event: &str, handler: EventFunction, thisobj: &T) {
        if false == self.event_listeners.contains_key(event) {
            self.event_listeners.insert(String::from(event), Vec::new());
        }

        let ptr = thisobj as *const _ as *mut usize;

        // println!("{:?}",thisobj.unwrap());

        let vec = self.event_listeners.get_mut(event).unwrap();

        let i = find(vec, handler, ptr);
        if i == -1 {
            vec.push(EventHandler {
                handler,
                thisobj: ptr,
            });
        }

        // println!("{:?}", vec);

        // let mut t:Vec<EventHandler> = Vec::new();
        // t.push(d);
    }

    #[allow(dead_code)]
    pub fn off<T>(&mut self, event: &str, handler: EventFunction, thisobj: &T) {
        if true == self.event_listeners.contains_key(event) {
            let vec = self.event_listeners.get_mut(event).unwrap();

            let ptr = thisobj as *const _ as *mut usize;
            let i = find(vec, handler, ptr);
            if i != -1 {
                vec.remove(i as usize);
            }
        }
    }


    #[allow(dead_code)]
    pub fn dispatch<T>(&mut self, event: &str, data: &T) {
        if true == self.event_listeners.contains_key(event) {
            let vec = self.event_listeners.get_mut(event).unwrap();
            for handler in vec.iter().clone() {
                let f = handler.handler;
                f(EventX {
                    event: String::from(event),
                    data: data as *const _ as *mut usize,
                });
            }
        }
    }

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
