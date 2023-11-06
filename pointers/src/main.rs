use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let config = AppConfig {
        a: 1,
        b: 2,
    };

    let c1 = Rc::new(RefCell::new(config));
    let c2 = c1.clone();

    println!("{:?}", c1);
    println!("{:?}", c2);

    // let pro = c1.borrow_mut();
    // pro.a = 99;
    c1.borrow_mut().a = 99;

    println!("{:?}", c2);
}

#[derive(Debug)]
struct AppConfig {
    a: i32,
    b: i32,
}

