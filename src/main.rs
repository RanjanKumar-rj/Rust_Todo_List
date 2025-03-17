use std::io;

fn get_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while taking input from user");
    let input: i32 = input.trim().parse().expect("Unable to parse input to i32");
    input
}

fn add_todo() {
    todo!()
}

fn list_todo() {
    todo!()
}

fn mark_as_done(){
    todo!()
}

fn main() {
    println!("Welcome to Todo App");
    loop {
        println!("Enter your choice");
        println!("1. Add Todo");
        println!("2. List Todo");
        println!("3. Mark as done");
        println!("4. Exit");

        let choice = get_input();
        match choice {
            1 => add_todo(),
            2 => list_todo(),
            3 => mark_as_done(),
            4 => {
                println!("Thanks for using Todo app");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
