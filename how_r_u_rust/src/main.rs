use std::cell::RefCell;
use std::fmt;


#[derive(Debug)]
struct Printable(i32);

struct DisplayPrintable(f32);

impl fmt::Display for DisplayPrintable{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DisplayPrintable({})", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        r#try!(write!(f, "["));
        for (ref c,ref v) in self.0.iter().enumerate() {
            r#try!(write!(f,"{0}:{1},",c,v));
        }
        write!(f, "]")
    }
}

fn main() {
    refcell_borrow();
    array();
    enum_size();
    test_internal_crate();
}

fn refcell_borrow(){
    let cell = RefCell::new(String::new());
    cell.borrow_mut().push('a');
    println!("{a} {printable:?} {dispp} {list}"
        , a = cell.borrow()
        , printable = Printable(32)
        , dispp = DisplayPrintable(2.0f32)
        , list = List(vec![1,2,3,4,5]));
}

fn array(){
    let xs : [u64;5] = [0;5];
    println!("{}", std::mem::size_of_val(&xs));
}

enum SizeTest{
    None , 
    IntSize(i32) , 
    IntDoubleSize(i32,i32),
    HuseSize{ _x : [i32;1024] }
}

fn enum_size(){
    println!("SizeTest::None {}", std::mem::size_of_val(&SizeTest::None));
    println!("SizeTest::IntSize {}", std::mem::size_of_val(&SizeTest::IntSize(1)));
    println!("SizeTest::IntDoubleSize {}", std::mem::size_of_val(&SizeTest::IntDoubleSize(2,3)));
    println!("SizeTest::HuseSize {}", std::mem::size_of_val(&SizeTest::HuseSize{ _x : [0; 1024]}));
}

extern crate how_r_u_rust;

fn test_internal_crate(){
    println!("{}", how_r_u_rust::hrn::something());
}

fn cast(){
    let s = String::from("s: &String");
    let i = s as u32;
}