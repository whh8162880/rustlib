use crate::event;

use event::MiniDispatcher;

pub trait Object<T> {
    fn new() -> Self;
    fn event(&mut self) -> &mut MiniDispatcher;
}

// impl<T: Object> A<T> {
//     pub fn new() -> Self {
//         Self { a: T::new() }
//     }
// }

// impl Object<B> for B {
//     fn new() -> Self {
//         Self {
//             _event: MiniDispatcher::new(),
//         }
//     }

//     fn event(&mut self) ->&mut MiniDispatcher<B> {
//         return &mut self._event;
//     }
// }

pub fn is_same<T>(left: &T, right: &T) -> bool {
    let a = left as *const _;
    let b = right as *const _;
    let r = a == b;
    println!("comparing: {:p} == {:p} -> {}", a, b, r);
    r
}

pub fn get_usize<T>(p: T) -> usize {
    return &p as *const _ as usize;
}
