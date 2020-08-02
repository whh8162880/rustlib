mod event;
mod object;

use event::{EventX, MiniDispatcher};

#[derive(Debug)]
pub struct Test {}
impl Test {
    pub fn new() -> Self {
        Test {}
    }

    pub fn abc(&mut self) {
        println!("hello");
    }
}

fn test(_p: &mut Test) {
    _p.abc();
}

fn main() {
    // let x = 5;
    // let y = &x;
    // println!("{}", *y);
    // println!("{:p}", y);
    // println!("{}", y);

    // unsafe {
    let mut d: MiniDispatcher = MiniDispatcher::new();

    
    let mut p = Test::new();

    
    
    

    test(&mut p);
    test(&mut p);

    // let a: i32 = 5;
    // let addr = &a as *const i32 as usize;
    // println!("addr：0x{:X}", addr);

    // let paddr = &p as *const _ as usize;

    let paddr = object::get_usize(p);

    // println!("paddr：0x{:X}", paddr);

    // let test = *paddr as Test;

    // println!("{:?}", addr);

    d.on(String::from("test"), event_handler, paddr);
    // d.on(String::from("test"), event_handler, paddr);

    // }

    // let b = *d;
    // *b.on(String::from("test2"), event_handler, None);

    // println!("{:?}", &d);
    // println!("{:?}", d)
}

fn event_handler(_event: EventX) {}
