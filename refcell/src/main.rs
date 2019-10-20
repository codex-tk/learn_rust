use std::cell::RefCell;
use std::sync::{Arc,Mutex};
use std::ops::Add;

#[derive(Debug)]
struct User{
    name: RefCell<Vec<String>>,
    name0: RefCell<String>,
}


fn apply<F>(f: F) where F: Fn() {
    f()
}


fn create_fn() -> impl Fn() -> i32 {
    || 81
}

fn main() {
    let user = User{ 
        name: RefCell::new(vec![String::from("name")]), 
        name0: RefCell::new(String::from("name0"))
    };

    {
        user.name.borrow_mut().push(String::from("32"));
        user.name0.borrow_mut().push_str("111");
        let mut s = user.name0.borrow_mut();
        *s = String::from("Change");
    }
    for v in user.name.borrow().iter() {
        print!("{}", v );
    }
    let s = String::from("1223");
    let s = s.add("aaa");
    println!("{}", s);

    let mtx_user = Mutex::new(User{ 
        name: RefCell::new(vec![String::from("name")]), 
        name0: RefCell::new(String::from("name0"))
    });
    for v in mtx_user.lock().unwrap().name.borrow().iter() {
        println!("{}", v );
    }

    let opt_user = Some(User{ 
        name: RefCell::new(vec![String::from("name")]), 
        name0: RefCell::new(String::from("name0"))
    });

    if let Some(ref u) = opt_user {
        println!("{:?}", u );
    }

    if let Some(u) = opt_user {
        println!("{:?}", u );
    }

    let print = || println!("print");
    apply(print);

    let cfn = create_fn();
    println!("{}", cfn() );

    let rfn = |&r| r;

    let v = vec![1,2,3];
    let v2 = v.iter().map(rfn).collect::<Vec<_>>();
    v.iter().for_each(|r| {
        println!("{}", *r );
    });
    println!("{:?}", v2);

    
}

