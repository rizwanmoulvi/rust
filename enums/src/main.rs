#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct Test {
    tup: (String, u32, bool, f64),
    name: String,
}

// Adding data to enum variant without struct
enum IpAdd {
    V4(u8, u8, u8, u8),
    V6(String),
    V10(Test),
}

//Adding wide variety of types embedded in its variants
enum Message {
    Quit, //no data associated with it at all
    Move { x: i32, y: i32}, // has nameed fields like struct
    Write(String), // has string
    ChangeColor(i32, i32, i32), // has 3 i32 variants
}

// Structs to do what is above doing
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

// Methods in Enum 
impl Message {
    fn call(&self) {
        println!("Called Enum");
    }
}


// Option Enum and its advantages over Null Values
// <T> means can hold one piece of data of any type
// Each type in <T> makes the overall Option<T> type a different type
enum Option<T> {
    None,
    Some(T),
}


fn main() {
    let t = Test {
        tup: (String::from("hello"), 10, true, 1.6),
        name: String::from("world"),
    };

    let t2 = Test {
        tup: (String::from("hey"), 5, false, 2.4),
        name: String::from("there"),
    };

    let x = t.tup.0;
    println!("{} {}",x, t.name);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}",four);
    println!("{:?}",six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let work = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let laptop = IpAdd::V4(255, 255, 255, 255);
    let mobile = IpAdd::V6(String::from("::127"));
    let iot = IpAdd::V10(t2);

    let m = Message::Write(String::from("happy"));   
    m.call();

    //Option
    let some_number = Some(5);
    let some_char = Some('e');

    // let absent_number: Option<_> = None;
    // Option<i8> is different from i8
}