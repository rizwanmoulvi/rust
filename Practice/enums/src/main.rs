#[derive(Debug)]

enum School {
    engineering(String),
    medical(String),
    business(String,)
}

enum Test<T> {
    None,
    Some(T),
}

fn main() {
    let s1 = School::engineering(String::from("CSE"));
    println!("{:?}", s1);

    let num: Test<i32> = Test::Some(10);
    
}