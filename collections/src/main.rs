fn test_vec(){
    let v = vec![1,2,3,4,5];
/*
    let first = match v.get(0) {
        Some(value) => value , 
        _ => 0 , // not works Option<&T> -> &T -> &i32
    };
    println!("{}", first );
    */
    if let Some(value) =  v.get(0) {
        println!("{}", value );
    }

    let mut v = vec![1,2,3,4,5];

    for i in &v {
        println!("{}", i);
    }

    for i in v.iter_mut() {
        println!("{}", *i);
        *i += 3
    }
    println!("{:#?}" , v);

    for i in &mut v {
        *i += 3;
    }
    println!("{:#?}" , v);
}

fn test_string(){
    let s = String::from("Hello");
    //let s = s + &String::from("World");
    let s = s + "World";
    println!("{}", s );

    let hello = "Здравствуйте";

    //let s = &hello[0..3];
    let s = &hello[0..4]; //Зд
    println!("{}", s );

    for c in hello.chars() {
        println!("{}", c);
    }
    for _b in hello.bytes() {
        //
    }
}

use std::collections::HashMap;

fn test_hashmap() {
    let mut map : HashMap<String,i32> = HashMap::new();
    map.insert(String::from("Key"), 10);
    if let Some(v) = map.get(&("Key".to_string())) {
        println!("{}" , v);
    } else {
        println!("none");
    }

    let teams = vec![String::from("blue") ,String::from("red")];
    let scores = vec![10,20];
    //let map : HashMap<&String,&i32>= teams.iter().zip(scores.iter()).collect();
    let map : HashMap<_,_>= teams.iter().zip(scores.iter()).collect();

    let mut map : HashMap<String,i32> = HashMap::new();
    map.insert(String::from("Key"), 10);
    map.entry("Key0".to_string()).or_insert(50);
    println!("{:?}", map );

    let mut_ref = map.entry("Key0".to_string()).or_insert(50);
    *mut_ref += 10;
    println!("{:?}", map );
}

fn main() {
    test_vec();
    test_string();
    test_hashmap();
}
