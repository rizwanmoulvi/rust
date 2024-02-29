fn main() {
    let mut s = String::from("hello, ");
    s.push_str("world");
    let _x  = s.clone();
    println!("{s}");

    // String Literal work without Double Free Error
    let mut a  = "Happy";
    let b = a;
    println!("{a}");

    let c: i32;

    take_ownership(s.clone());
    println!("{s}");

    let s1 = gives_ownership();
    let s2 = gives_ownership();
    println!("{s2}");
    println!("{s1}");

    let s3 = takes_and_give_ownership(s2);
    println!("{s3}");

    let s4 = String::from("Cool");
    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}", s5, len);
    
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let c = String::from("Hey there");
    c
}

fn takes_and_give_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: String) -> (String, usize) {
    let len = a_string.len();
    (a_string, len)
}