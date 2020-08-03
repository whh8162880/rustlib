mod event;
mod object;

use event::{EventX, MiniDispatcher};

#[derive(Debug)]
pub struct Test {
    v: i32,
}
impl Test {
    pub fn new() -> Self {
        Test { v: 3 }
    }

    pub fn abc(&mut self) {
        println!("hello");
    }
}

fn _test_mini_dispatcher() {
    let mut d: MiniDispatcher = MiniDispatcher::new();
    let test = Test::new();
    let ptr = object::_get_mut_usize(test);
    d.on(String::from("test"), _event_handler, ptr);
    d.on(String::from("test"), _event_handler, ptr);
}

fn main() {
    let obj = object::Object::new();
    // let o = unsafe { std::mem::transmute::<*mut object::Object, &mut object::Object>(obj) };
    

    println!("{:?}", obj);
    // println!("{:?}", o);

    // let x = 5;
    // let y = &x;
    // println!("{}", *y);
    // println!("{:p}", y);
    // println!("{}", y);

    // unsafe {
    // let d: MiniDispatcher = MiniDispatcher::new();

    // let p = Test::new();

    // let ptr = &p as *const _ as *mut Test;
    // let ptr2 = &p as *const _ as *mut usize;
    // println!("{:?}", ptr);
    // println!("{:?}", ptr2);

    // let ptr = &p as *const _ as *mut Test;
    // let v = unsafe { &mut *ptr };
    // v.abc();
    // println!("{:?}", v);

    // let ptr = &p as *const _ as *mut usize;
    // let v = unsafe { std::mem::transmute::<*mut usize, &mut Test>(ptr) };
    // v.abc();
    // println!("{:?}", v);

    // let ptr2 = &p as *const _ as usize;

    // std::mem::transmute::<*mut usize,Test>(ptr)

    // let t =object::get_unsafe_mut_usize_value::<&mut Test>(ptr);
    // t.abc();

    // let t = unsafe { std::mem::transmute::<*mut usize, &mut Test>(ptr) };
    // t.abc();

    // println!("{:?}", t);
    // println!("{:?}", unsafe { std::mem::transmute::<*mut usize,&Test>(ptr) });
    // println!("{:?}", unsafe { std::mem::transmute::<*mut usize,&Test>(ptr) });

    // test(&mut p);
    // test(&mut p);

    // let a: i32 = 5;
    // let addr = &a as *const i32 as usize;
    // println!("addr：0x{:X}", addr);
    // let paddr = &p as *const _ as usize;
    // let paddr = object::get_mut_usize(p);

    // println!("paddr：0x{:X}", paddr);

    // let test = *paddr as Test;

    // println!("{:?}", addr);

    // d.on(String::from("test"), event_handler, ptr);

    // d.on(String::from("test"), event_handler, paddr);

    // }

    // let b = *d;
    // *b.on(String::from("test2"), event_handler, None);

    // println!("{:?}", &d);
    // println!("{:?}", d)
}

// #[derive(Debug)]
//     pub struct A {
//         v: i32,
//     }

// impl A {
//     pub fn new() -> Self {
//         A { v: 3 }
//     }

//     pub fn test(){

//     }
// }

// fn main2() {

//     let a = A{v:3};

//     let ptr = &a as *const _ as *mut A;
//     let v = unsafe { &*ptr };

//     let ptr2 = &a as *const _ as *mut usize;
//     let v1 = unsafe { std::mem::transmute::<*mut usize, &mut A>(ptr2) } as &mut A;

// }

fn _event_handler(_event: EventX) {}
