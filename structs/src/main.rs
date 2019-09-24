// structs
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User{
    fn print(&self){
        println!("{} {} {} {}", self.username , self.email , self.sign_in_count , self.active );
    }
}

struct Color(i32,i32,i32);

// enums 
enum IpAddrKind{
    V4(String),
    V6(String),
}

#[repr(C)]
struct buf{
    ptr: i32,
    size: i32,
}

fn main() {
    let user1 = User{
        username: String::from("name"),
        email: String::from("email") ,
        sign_in_count: 1 ,
        active: false ,
    };
    

    let user2 = User {
        username: String::from("name2"),
        email: String::from("email2") ,
        ..user1
    };

    println!("{:#?} {:#?}", user1 , user2 );
    user1.print();

    let _black = Color(0,0,0);

    let loopback = IpAddrKind::V4(String::from("127.0.0.1"));

    if let IpAddrKind::V4(ref addr) = loopback {
        println!("{}", addr );
    }

    match loopback {
        IpAddrKind::V4(ref addr) => println!("{}", addr ) ,
        _ => ()
    };

    let option_value = None;//Some(5);
    let some_fn = Some;
    let some_value = some_fn(4);
    match some_value {
        Some(v) => println!("{} ", v ),
        None =>{},
    };
    let v = match option_value {
        Some(v) => v,
        None => 0,
    };
    println!("{}", v );
}
