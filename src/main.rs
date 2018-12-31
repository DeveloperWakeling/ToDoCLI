use std::env;

struct ToDoItem {
    name: String,
    completed: char
}

impl ToDoItem {
    fn new(name: String) -> ToDoItem {
        ToDoItem {
            name,
            completed: ' '
        }
    }
}

struct ToDoList {
    list: Vec<ToDoItem>
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList{
            list: Vec::new()
        }
    }
    fn add_to_list(&mut self, name: String) {
        self.list.push(ToDoItem::new(name));
    }

    fn complete(&mut self, index: usize){
        self.list[index].completed = 'x';
    }
    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}. [{}] - {}", index, item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String),
    Complete(usize)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let cmd = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "complete" => Command::Complete(arguments[2].parse().unwrap()),//Unwrap just gets the Ok
        _ => panic!("You must provide a proper command")//_ is basically the defualt
    };
    let mut todo_list = ToDoList::new();
    
    todo_list.add_to_list("Say Hi".to_string());
    todo_list.add_to_list("Say Hello".to_string());

    match cmd {
        Command::Get => todo_list.print(),
        Command::Add(name) => {
            todo_list.add_to_list(name);
            todo_list.print();
        },
        Command::Complete(index) => {
            todo_list.complete(index);
            todo_list.print();
        }
    }
}
