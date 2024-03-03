#[derive(Debug)]
struct Book {
    name: String,
    quantity: u32,
    rack: char,
    available: bool,
}

impl Book {
    fn new(name: String, quantity: u32, rack: char, available: bool ) -> Self {
        Self {
            name: name,
            quantity: quantity,
            rack: rack,
            available: available,
        }
    }

    fn print(&self) {
        println!("The name of book is: {}", self.name);
        println!("Total quantity of book is: {}", self.quantity);
        println!("The rack of book is: {}", self.rack);
        println!("Book is available? : {}", self.available);
    }

}

fn main() {

    let book1 = Book::new(String::from("Lupin"), 10, 'A', true);

    // dbg!(book1);
    book1.print();
}



