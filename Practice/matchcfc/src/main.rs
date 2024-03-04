#[derive(Debug)]
enum Drink {
    Pepsi,
    Coke,
    Limca,
    Redbull(Taste),
    Monster,
}

#[derive(Debug)]
enum Taste {
    Original,
    Orange,
    Watermelon,
    Lime,
}

fn price(drink: Drink) -> u32 {
    match drink {
        Drink::Pepsi => 25,
        Drink::Coke => 20,
        Drink::Limca => 30,
        Drink::Redbull(taste) => {
            match taste {
                Taste::Original => 100,
                Taste::Orange => 120,
                Taste::Watermelon => 125,
                Taste::Lime => 110,
            }
        }
        Drink::Monster => 105,
    }
}

fn token(bill: Option<u32>) -> Option<u32> {
    match bill {
        None => None,
        Some(i) => Some(i),
    }
} 

fn main() {
    let a = price(Drink::Pepsi);
    println!("The value of Pepsi is: {}", a);

    let b = price(Drink::Coke);
    println!("The value of Coke is {}", b);

    let c = price(Drink::Limca);
    println!("THe value of Limca is {}", c);

    let d = price(Drink::Redbull(Taste::Watermelon));
    println!("The value of Redbull (Watermelon) is {}", d);
    
    let e = price(Drink::Redbull(Taste::Original));
    println!("The value of Redbull (Original) is {}", e);

    let f = price(Drink::Monster);
    println!("The value of Monster is {}", f);

    let ten = Some(10);
    let bill = token(ten);
    let none = token(None);


    
}