fn main() {

    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("user123@email.com"),
        sign_in_count: 25,
    };

    let active: bool = user1.active;
    println!("{active}");

    let user2 = User {
        active: user1.active,
        username: String::from("newemail@email.com"),
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("new2@email.com"),
        ..user2
    };

    let user4 = User {
        active: false,
        sign_in_count: 0,
        ..user3
    };

    let black = color(0, 0, 0);
    let origin = point(0, 0, 0);

    let a = black.2;
    println!("{a}");

    let subject = AlwaysEqual;

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User {
    User {
        active: active,
        username: username,
        email: email,
        sign_in_count: sign_in_count,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 10,
    }
}

// Tuple Structs

struct color(i32, i32, i32);
struct point(i32, i32, i32);

// Unit  Like Struct

struct AlwaysEqual;