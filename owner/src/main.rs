fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}" , s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{} {}" , s1 , s2);

    take_ownership(s2);
    //println!("{}" , s2);

    let x : u32 = 5;
    make_copy(x);

    let s1 = take_and_give_back(s1);
    println!("{}" , s1);

    println!("{}", calculate_length(&s1));

    let mut s = String::from("Hello");
    append(&mut s, ", World!");
    println!("{}" , s);

    let borrow1 : &mut String = &mut s;
    append(borrow1," Test");

    //let borrow2 = &mut s; // can't borrow mut ref twice
    //append(borrow2,"Test");
    
    println!("{}" , s);

    let first_w = first_word(&s[..]);
    //s.clear(); // first_w borrow s
    println!("{}", first_w);
}

fn take_ownership(_:String){}

// Copy trait
fn make_copy(_:u32){}

fn take_and_give_back(s:String) -> String {
    s
}

fn calculate_length(s:&String) -> usize {
    s.len()
}

fn append(s:&mut String , a: &str) {
    s.push_str(a);
}

fn first_word(s:&str) -> &str {
    for (i,&item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}