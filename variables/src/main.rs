const MAX_POINTS : u32 = 100_000;

fn main() {
    immutable_test();
    shadow();
    tuples();
    test_if();
    for_();
}

fn immutable_test(){
    //let x = 5;
    let mut x = 5;
    println!("{}" , x );
    x = 6;
    println!("{}" , x );
}

fn shadow(){
    let x = 10;
    println!("{}" , x );
    let x = x + 1;
    println!("{}" , x );
    let x = x + 2;
    println!("{}" , x );
}

fn tuples(){
    let tup : (i32,f64,u8) = (400,6.5,1);
    println!("{}" , tup.1 );
}

fn test_if(){
    let cond = true;
    let value = if cond {
        4
    } else {
        5
    };
    println!("{}" , value );
}

fn for_(){
    let a = [1,2,3,4,5];
    for e in a.iter() {
        println!("{}" ,e );
    }

    for v in (1..5).rev() {
        println!("{}" , v);
    }
}