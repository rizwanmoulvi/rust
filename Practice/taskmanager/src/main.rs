#[derive(Debug)]

struct Task {
    id: u32,
    name: String,
    description: String,
    status: bool,
}

impl Task {
    fn new(id: u32, name: String, description: String, status: bool ) -> Self{
        Self {
            id: id,
            name: name,
            description: description,
            status: status,
        }
    }

    fn update_status(&mut self, status: bool) {
        self.status = status;
    }
}

fn main() {
    let mut task1 = Task::new(
        1, 
        String::from("Homework"), 
        String::from("Do the homework of CSE"), 
        false
    );

    println!("The id of task 1 is : {}", task1.id);
    println!("The name of task 1 is : {}", task1.name);
    println!("The description of task 1 is : {}", task1.description);
    println!("The status of task 1 is : {}", task1.status);

    task1.update_status(true);
    println!("The status of task 1 is : {}", task1.status);
}