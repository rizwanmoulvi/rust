//Match to compare a values agaisnt series of patterns
//'if' control flow takes boolean while 'match' takes any type
//match arm has two parts 1.pattern and 2.expression and operator '=>'
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Paisa,
}

//Patterns that Bind to Values
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Florida,
    Arizona,
    Texas,
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// matches options are limited and we have to cover all options
// fn minus_one(y: Option<i32>) -> Option<i32> {
//     match y {
//         Some(i) => Some(i -1), //causes error because there is not None
//     }
// }




fn in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
        Coin::Paisa => {
            println!("Paisa is unit of INR not USD!");
            0
        },
    }
}

fn main() {
    
    let a = in_cents(Coin::Penny);
    println!("The value of Penny in cents is {}",a);

    let b = in_cents(Coin::Nickel);
    println!("The value of Penny in cents is {}",b);

    let c = in_cents(Coin::Dime);
    println!("The value of Dime in cents is {}",c);

    let d = in_cents(Coin::Quarter(UsState::Alaska));
    println!("The value of Quarter in cents is {}", d);

    let e = in_cents(Coin::Paisa);
    println!("{}", e);

    let five = Some(4);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);

    // Catch-all Patterns and the_Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => move_player(other), //covers all other possibilities ('other is just place holder')
        _ => reroll(), //we can use this to ignore all other values 
        // _ => (), // Nothing happens, we are not going to use any other value
        //if we add 'other' on top then other conditions don't execute
    }
    fn add_hat() {}
    fn remove_hat() {}
    fn move_player(space: u8) {}
    fn reroll() {}
}
