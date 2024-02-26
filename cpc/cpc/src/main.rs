use std::io;

fn main() {
    let x = 5;
    println!("{x}");
    // x = 6; // changing the value of variable if it is mutable
    let x = x + 3; //over shadowing the previous declartion of variable
    println!("{x}");

    const Y: i32 = 10;
    println!("{Y}");

    {
        let x = 7;
        println!("The value of inner X: {x}");
        
        println!("{Y}");
    }
    
    println!("{x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
    
    let p = 4;
    let q = 43;
    
    println!("{}",p+q);

    let v: i128 = 44;
    println!("{v}");

//_____________________________________________

    //Data Types
        // Two Types

            //1. Scalar
                //Integer
                //Float
                //Booleans
                //Characters
    
    // Integer
    let a: i8 = 8;
    let a: i128 = -128;

    let b: u8 = 8;
    let b: u128 = 128;

    let c = 64; //by default architecture bits: i64 or i32

    // Integer overflow, gets wrapped in --release
    // let u: i8 = 150;
    // println!("{u}");

    // FLoat only f32 and f64
    let d: f32 = 2.4; // single precision i.e 7 digit after decimal point
    println!("{d}");

    let e: f64 = 3.23; // single precision i.e 15 digit after decimal point
    println!("{e}");

    // Cannot operate on different data types
    // let sum = a+b;
    // println!("{sum}");

    // Boolean
    let f = true;
    let g: bool = false;

    // Character: 4 bit, represents more than ASCII
    let mut h = 'h';
    let i: char = 'i';

    let mut j = String::new();

    io::stdin().read_line(&mut j).expect("Enter a char");
    println!("{j}");


//------------------------------------------------------
    // Compound Types
    
    //Tuple: Can different types
    let k = (500, 5.2, -8);

    let l: (i32, f64, u8) = (50, 1.1, 62);

    let (x, y, z) = l; // Destructuring of a Tuple to access elements
    println!("The value of y is: {y}");

    let x = l.2;
    println!("The value at 2 index of tuple l is : {x}");

    let m: () = (); //unit
    
    //Array: Only same type of elements are allowed

    let n = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", 
                 "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let m = months[1];
    println!("{m}");

    let o: [f32; 4] = [0.1, 0.2, 0.3, 0.4];

    let p = [7; 5]; // p = [7,7,7,7,7]
    println!("{}", p[3]);

    let q = [10, 20, 30, 40, 50];

    println!("Enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to take input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index enetered was not a number");

    let element = q[index];

    println!("The value at index {index} is {element}");

}