use std::mem;
use std::ffi;

#[derive(Debug)]
struct Dropped(bool);

#[derive(Debug)]
struct Dropable<'a>(&'a mut Dropped);

impl Drop for Dropable<'_> {
    fn drop(&mut self){
        (self.0).0 = true;
        println!("Drop");
    }
}

fn main() {
    let mut dropped = Dropped(false);
    println!("Hello, world!");
    {
        //let mut raw_ptr : *const ffi::c_void = std::ptr::null();
        let mut raw_ptr :*const Dropable = std::ptr::null();
        {
            let d = std::sync::Arc::new(Dropable(&mut dropped));
            //raw_ptr = std::sync::Arc::into_raw(d) as * const ffi::c_void;
            raw_ptr = std::sync::Arc::into_raw(d);
        }

        let d = unsafe {
            std::sync::Arc::from_raw(raw_ptr as *const Dropable)
        };
        println!("{:?}", d);
        
    }

    if dropped.0 {
        println!("Dropped");
    }
    println!("Bye, world!");
}
