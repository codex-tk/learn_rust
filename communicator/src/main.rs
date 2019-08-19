extern crate communicator;

pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {
        println!("middle_secret_function");
    }

    pub mod inside {
        pub fn inner_function() {
            super::middle_secret_function();
            super::inside::secret_function();
        }

        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

enum TrafficLight {
    R(i32,i32,i32) , G(i32,i32,i32) , B(i32,i32,i32) ,
}

use TrafficLight::{R,G};

fn print_color_code(tr : TrafficLight ) {
    match tr {
        TrafficLight::R(a,b,c) => {
            println!("R {} {} {} ", a,b,c);
        } ,
        TrafficLight::G(a,b,c) => {
            println!("G {} {} {} ", a,b,c);
        } ,
        TrafficLight::B(a,b,c) => {
            println!("B {} {} {} ", a,b,c);
        } ,
    };
}

fn main(){
    communicator::client::connect();
    try_me();

    let g = TrafficLight::G(1,2,3);
    print_color_code(g);
    
    let red = R;
    print_color_code(red(3,4,5));
    
}