use std::io;

fn main() {
    println!("Enter the number of terms:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to take input");

    let n: u32 = n.trim().parse().expect("Not a number");

    // Fib fast
    let x = fib();
    let mut a = 0;
    for i in x {
        println!("{} : {}",a, i);
        a += 1;
    }
    println!("End of fib fib fast");
    //Fib Slow
    for i in 0..n {
        let x = fib1(i);
        println!("{} : {}",i,x);
    }
}

fn fib1(n: u32) -> u32 {
    if n == 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    return fib1(n-1) + fib1(n-2)
}

fn fib() -> [usize; 50] {
    let mut arr: [usize; 50] = [0; 50];
    arr[0] = 0;
    arr[1] = 1;
    for i in 2..50 {
        arr[i] = arr[i-1] + arr[i - 2];
    }
    return arr
}