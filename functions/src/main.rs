fn main() {

    new_fun(5.3,'i');
    println!("Hello, world!");

    let x = 5;
    let y = x;
    println!("{y}");

    let a = {
        let b = 3;
        b+1
    };

    let sum = y+x;
    println!("{sum}");

    println!("{a}");
    
    let r = five();
    println!("{r}");

    let v = plus_one(7);
    println!("{v}");
}

fn new_fun(value: f32, metrics: char) {
    println!("Greeting from new function");
    println!("The height of box is {value}{metrics}");
}

fn five() -> f32 {
    5.6
    // return 9
}

fn plus_one(x: i32) -> i32 {
    x+1
}