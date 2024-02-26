fn main() {
    let number = 30;

    if number < 5 {
        println!("Good boy");
    } else if number > 10 {
        println!("Really Bad Boy");
    } else {
        println!("Bad boy");
    }

    let x = 10;

    let state = if x == 10 { 5 } else { 6 }; //{6.3} not possible 
    println!("{state}");


    // Loop 
    loop {
        println!("a");
        // break;
        // let x = 5;
        // if x == 5 {
        //     println!("b");
        //     continue;
        // }
        break;
    }

    // Returning value from loop

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");
        if counter == 5 {
            break counter * 2;
        }
    };

    println!("{result}");

    // Loop Labels

    let mut count = 0;

    'outer_loop: loop {
        println!("count = {count}");

        let mut rem = 10;

        'inner_loop: loop {
            println!{"remaining = {rem}"};

            if rem == 9 {
                break 'inner_loop;
            }
            if count == 2 {
                break 'outer_loop;
            }
            rem -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While loop

    let mut n = 3;

    while n != 0 {
        println!("{n}");

        n -= 1;
    }

    // While loop on collection
    let a  = [10, 20, 30, 40, 50];

    let mut i = 0;

    while i < a.len() {
        println!("the value is : {}", a[i]);
        
        i += 1;
    }

    // For loop

    for i in a {
        println!("for value : {i}");
    }

    for n in 1..4 {
        println!("{n}");
    }

    for n in (4..9).rev() {
        println!("rev value: {n}");
    }

}