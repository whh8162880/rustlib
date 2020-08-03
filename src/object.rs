use crate::event;

// pub trait IObject<T> {
//     fn new() -> Self;
//     fn event(&mut self) -> &mut MiniDispatcher;
// }

#[derive(Debug)]
pub struct Object {
    __obj_id: *mut usize,
}

impl Object {
    pub fn new() -> Self {
        let mut p = Self { __obj_id: &mut 0 };
        p.create();
        return p;
    }

    fn create(&mut self) {
        self.__obj_id = &self as *const _ as *mut usize;
    }
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

pub fn _is_same<T>(left: &T, right: &T) -> bool {
    let a = left as *const _;
    let b = right as *const _;
    let r = a == b;
    println!("comparing: {:p} == {:p} -> {}", a, b, r);
    r
}

pub fn _get_mut_usize<T>(p: T) -> *mut usize {
    return &p as *const _ as *mut usize;
}

// pub fn get_unsafe_mut_usize_value<T>(ptr: *mut usize) -> T{
//     let v = unsafe { std::mem::transmute::<*mut usize, T>(ptr) };
//     return v;
// }
