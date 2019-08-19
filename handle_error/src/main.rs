use std::fs::File;
use std::io::ErrorKind;

fn test_result() -> Result<File,std::io::Error> {
    let f = match File::open("hello.txt") {
        Ok(file) => file ,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc ,
                Err(e) => {
                    panic!("{:?}" , e);
                },
            }
        } ,
        Err(error) => {
            panic!("{:?}" , error);
        },
    };
    // simpl panic .unwrap() or .expect("msg")
    let f = File::open("hello.txt").unwrap();

    // error propagation
    // 1.
    let f = File::open("hello.txt")?;
    // 2.
    let f = match File::open("hello.txt") {
        Ok(file) => file ,
        Err(error) => {
            return Err(error);
        },
    };
    // normal case
    Ok(f)
    /*
    match File::open("hello.txt") {
        Ok(file) => Ok(file) ,
        Err(error) => Err(error) ,
    }*/
}

fn main() {
    test_result();
}
