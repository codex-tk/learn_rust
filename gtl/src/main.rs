fn largest<T :  Copy + std::cmp::PartialOrd >(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn point_test(){
    let integer = Point{ x: 5, y: 10};
    let complex = Point{ x: 1, y: 1.1};
    println!("{:#?} {}", complex ,complex.get_x());
}

fn some_func<T,U>(t: T,u: U)
    where T: Copy + std::cmp::PartialOrd ,
          U: Copy + std::cmp::PartialOrd
{
    print!("SomeFunc");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn main() {
   let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    point_test();

    some_func(1,1.2);

    test_longest();
}