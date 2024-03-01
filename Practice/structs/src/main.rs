fn main() {
    let s1 = Student {
        name: String::from("Peter"),
        age: 19,
        active: true,
        section: String::from("K22LQ"),
        year: 2,
    };

    println!("Student's details are {:#?}", s1);


}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    active: bool,
    section: String,
    year: u32,
}

