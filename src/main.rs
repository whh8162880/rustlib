mod event;
mod object;
mod date;
mod amf;

use event::{EventX, MiniDispatcher};
// use futures::executor::block_on;

use futures::executor::block_on;

#[allow(unused_imports)]
use std::time::{Duration, Instant};
// use std::{sync::Mutex, collections::HashMap};
// use amf::AMFData;
// use serde_amf::AMFData;

#[derive(Debug,Clone)]
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

#[allow(dead_code)]
fn test_mini_dispatcher() {
    let mut d: MiniDispatcher = MiniDispatcher::new();
    let test = Test { v: 3 };
    d.on("test", event_handler, &test);
    d.on("test2", event_handler_2, &test);
    d.dispatch("test", &test);
    d.dispatch("test2", &1);
    // d.off("test", event_handler, &test);
    // d.dispatch("test", &test);
}

#[allow(dead_code)]
fn event_handler(event: EventX) {
    let data = unsafe { std::mem::transmute::<*mut usize, &mut Test>(event.data) };
    data.abc();
    println!("event data {:?}", data);
}

#[allow(dead_code)]
fn event_handler_2(event: EventX) {
    let data = unsafe { std::mem::transmute::<*mut usize, &mut i32>(event.data) };
    println!("event data {:?}", data);
}

// struct Foo<'a>{
//     i:&'a i32,
// }

// fn global_data() -> &'static Mutex<HashMap<i32, String>> {
//     static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
//     INSTANCE.get_or_init(|| {
//         let mut m = HashMap::new();
//         m.insert(13, "Spica".to_string());
//         m.insert(74, "Hoyten".to_string());
//         Mutex::new(m)
//     })
// }
#[allow(unused_must_use)]
fn main() {
    // test_mini_dispatcher();
    block_on(date::test());
    // date::test();
    // let test = Test::new();
    // let mut now = Instant::now();

    // for _i in 0..10000000 {
    //     // let p = &test as *const _ as *mut usize;
    // }

    // println!("{}", now.elapsed().as_millis());

    // now = Instant::now();


    // for _i in 0..10000000 {
    //     unsafe { std::mem::transmute::<*mut usize, &mut Test>(&test as *const _ as *mut usize) };
    // }

    // println!("{} ", now.elapsed().as_millis());
    // println!("{:?}",unsafe { std::mem::transmute::<*mut usize, &mut Test>(&test as *const _ as *mut usize)});

    
    // let test2 = &test;

    // let ptr = &test as *const _ as *mut usize;
    // let ptr2 = test2 as *const _ as *mut usize;

    // println!("{:?}  {:?}",ptr,ptr2);

    // let mut obj = object::Object::new();
    // println!("{:?}", obj);
    // // let o = unsafe { std::mem::transmute::<*mut object::Object, &mut object::Object>(obj) };
    // let test = Test{v:3};
    // println!("{:?}", test);
    // obj.add_component("test", "Test", test);

    // println!("object : {:?}", obj);

    // let t = obj.get_component::<Test>("test").ok().unwrap();
    // println!("{:?}", t);

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
