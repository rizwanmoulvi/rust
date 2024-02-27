use std::io;

fn main() {
    println!("Control Flow Practice");
    'outer: loop {
        println!("Enter Your Age");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to take input");
        let age: i32 = age.trim().parse().expect("Age provided is not number");

        if age > 18 {
            println!("You are eligible!");
            let mut center: [i32; 4] = [0,0,0,0];
            for i in 0..4 {
                println!("Enter {i} number: ");
                let mut x = String::new();
                io::stdin()
                    .read_line(&mut x)
                    .expect("Failed to take input");

                let x: i32 = x.trim().parse().expect("Wrong input");
                center[i] = x;
            }
            // println!("{}",center[2]);
            for i in center {
                println!("The first number is: {}", i);
            }
            break 'outer;
        } else {
            println!("You are not eligible, you are punished");
            let mut counter = 5;
            println!("Say sorry {counter} times");

            while counter > 0 {
                let mut x = String::new();
                io::stdin()
                    .read_line(&mut x)
                    .expect("Wrong input");
                println!("That's a {x} say again");
                if counter == 3 {
                    println!("I Forgive you");
                    break 'outer
                }
                counter -= 1;
            }
        }
    }

}
