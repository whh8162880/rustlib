use std::collections::HashMap;
// pub trait IObject<T> {
//     fn new() -> Self;
//     fn event(&mut self) -> &mut MiniDispatcher;
// }

#[derive(Debug)]
pub struct Component {
    instance: String, // 组件名
    name: String,     // 组件变量名
    ptr: *mut usize,  // 组件内存地址
}

#[derive(Debug)]
pub struct Object {
    __obj_id: *mut usize,
    components: HashMap<String, Component>,
}

impl Object {

    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut p = Self {
            __obj_id: &mut 0,
            components: HashMap::new(),
        };
        p.create();
        return p;
    }

    fn create(&mut self) {
        self.__obj_id = &self as *const _ as *mut usize;
    }

    #[allow(dead_code)]
    pub fn add_component<T>(&mut self, name: &str, instance: &str, target: T) {
        let ptr = &target as *const _ as *mut usize;
        if true == self.components.contains_key(name) {
            self.components.remove(name);
        }
        self.components.insert(
            String::from(name),
            Component {
                name: String::from(name),
                ptr,
                instance: String::from(instance),
            },
        );
    }

    #[allow(dead_code)]
    pub fn remove_component(&mut self, name: &str) {
        if true == self.components.contains_key(name) {
            self.components.remove(name);
        }
    }

    #[allow(dead_code)]
    pub fn get_component<T>(&mut self, name: &str) -> Result<&mut T, String> {
        let error = String::from("no exist");
        let v: &mut T;
        if true == self.components.contains_key(name) {
            let c = self.components.get_mut(name).unwrap();
            v = unsafe { std::mem::transmute::<*mut usize, &mut T>(c.ptr) };
            Ok(v)
        } else {
            Err(error)
        }
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

// pub fn get_unsafe_mut_usize_value<T>(ptr: *mut usize) -> &mut T{
//     let v = unsafe { std::mem::transmute::<*mut usize, &mut T>(ptr) };
//     return v;
// }
