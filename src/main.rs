use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct ValidationErrors {
    title_errors: Vec<String>,
    id_errors: Vec<String>,
    status_errors: Vec<String>,
}

impl ValidationErrors {
    pub(crate) fn is_empty(&self) -> bool {
        self.title_errors.is_empty() && self.id_errors.is_empty() && self.status_errors.is_empty()
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = String::new();
        if !self.title_errors.is_empty() {
            errors += "Title errors:\n";
            for err in &self.title_errors {
                errors += err;
            }
        }
        if !self.id_errors.is_empty() {
            errors += "ID errors:\n";
            for err in &self.id_errors {
                errors += err;
            }
        }
        if !self.status_errors.is_empty() {
            errors += "Status errors:\n";
            for err in &self.status_errors {
                errors += err;
            }
        }
        write!(f, "{}", errors)
    }
}

impl Error for ValidationErrors {}

fn validate_todo(todo: &Todo) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors {
        title_errors: Vec::new(),
        id_errors: Vec::new(),
        status_errors: Vec::new(),
    };

    // Validate title
    if todo.title.is_empty() {
        errors.title_errors.push("Title cannot be empty.".to_string());
    }
    if todo.title.len() > 100 {
        errors.title_errors.push("Title cannot exceed 100 characters.".to_string());
    }

    // Validate ID (assuming ID is an integer)
    if todo.id < 0 {
        errors.id_errors.push("ID cannot be negative.".to_string());
    }

    // Validate status (assuming status is a boolean)
    // No specific validation needed for boolean status

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    status: bool,
}

impl Todo {
    fn new(id: i32, title: String) -> Result<Todo, ValidationErrors> {
        let todo = Todo {
            title: title.to_string(),
            id,
            status: false,
        };

        validate_todo(&todo)?;

        Ok(todo)
    }

    fn toggle(&mut self) {
        self.status = !self.status;
    }

    fn print(&self) {
        println!("ID: {}, Title: {}, Status: {}", self.id, self.title, self.status);
    }
}

struct TodoManager {
    todos: Vec<Todo>,
    title_index: HashMap<String, usize>,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TodoManager {
    fn new() -> TodoManager {
        TodoManager {
            todos: Vec::new(),
            title_index: HashMap::new(),
        }
    }

    fn add(&mut self, title: String) -> Result<(), String> {
        let id = self.todos.len() as i32;
        let todo = match Todo::new(id, title.clone()) {
            Ok(todo) => todo,
            Err(err) => {
                return Err(format!("Validation error: {}", err));
            }
        };
        self.todos.push(todo);
        self.title_index.insert(title, self.todos.len() - 1);
        Ok(())
    }

    fn remove_by_status(&mut self, status: bool) {
        self.todos.retain(|todo| todo.status != status);
    }

    fn remove_by_id_range(&mut self, min: i32, max: i32) {
        for id in min..=max {
            if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
                self.todos.remove(index);
            }
        }
    }

    fn remove_all_todos(&mut self) {
        self.todos.clear();
        self.title_index.clear();
    }
    fn print_by_title(&self, title: &str) {
        if let Some(index) = self.title_index.get(title) {
            self.todos[*index].print();
        }
    }

    fn find_by_id(&mut self, id: i32) -> Option<&mut Todo> {
        self.todos.get_mut(id as usize)
    }

    fn remove_by_id(&mut self, id: i32) -> Result<(), String> {
        if let Some(index) = self.todos.get_mut(id as usize) {
            let title = index.title.clone();
            self.todos.remove(id as usize);
            self.title_index.remove(&title);
            Ok(())
        } else {
            Err(format!("Todo with ID {} not found.", id))
        }
    }

    fn toggle_by_id(&mut self, id: i32) -> Result<(), String> {
        if let Some(todo) = self.find_by_id(id) {
            todo.toggle();
            Ok(())
        } else {
            Err(format!("Todo with ID {} not found.", id))
        }
    }

    fn find_by_title(&self, title: &str) -> Result<&Todo, String> {
        if let Some(index) = self.title_index.get(title) {
            Ok(&self.todos[*index])
        } else {
            Err(format!("Todo with title '{}' not found.", title))
        }
    }

    fn remove_by_title(&mut self, title: &str) -> Result<(), String> {
        if let Some(index) = self.title_index.get(title) {
            self.todos.remove(*index);
            self.title_index.remove(title);
            Ok(())
        } else {
            Err(format!("Todo with title '{}' not found.", title))
        }
    }

    fn toggle_by_title(&mut self, title: &str) -> Result<(), String> {
        if let Some(index) = self.title_index.get(title) {
            self.todos[*index].toggle();
            Ok(())
        } else {
            Err(format!("Todo with title '{}' not found.", title))
        }
    }

    fn find_by_status(&self, status: bool) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.status == status).collect()
    }

    fn toggle_by_status(&mut self, status: bool) {
        self.todos.iter_mut().for_each(|todo| todo.status = status);
    }

    fn print(&self) {
        for todo in &self.todos {
            todo.print();
        }
    }

    fn print_by_id(&mut self, id: i32) {
        if let Some(todo) = self.find_by_id(id) {
            todo.print();
        } else {
            println!("Todo with ID {} not found.", id);
        }
    }

    fn print_by_status(&self, status: bool) {
        for todo in self.find_by_status(status) {
            todo.print();
        }
    }

    fn save_to_json(&self, filename: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.todos)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load_from_json(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let todos: Vec<Todo> = serde_json::from_str(&contents)?;
        self.todos = todos;
        self.title_index = self.todos.iter().enumerate().map(|(i, todo)| (todo.title.clone(), i)).collect();
        Ok(())
    }
}

fn take_input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_input_to_cmd(input: &str) -> (&str, &str) {
    let mut parts = input.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");
    (cmd, args)
}

fn interpret_cmd(cmd: &str, args: &str, todo_manager: &mut TodoManager) {
    let num_args = args.chars().count();

    match cmd {
        "add" => {
            if num_args == 0 {
                println!("Please provide a title");
                return;
            }
            match todo_manager.add(args.to_string()) {
                Ok(_) => println!("Todo added successfully."),
                Err(e) => println!("{}", e),
            }
        },

        "remove_all" => {
            todo_manager.remove_all_todos();
        },

        "remove_by_id" => {
            if num_args == 0 {
                println!("Please provide an id");
                return;
            }
            match todo_manager.remove_by_id(args.parse().unwrap()) {
                Ok(_) => println!("Todo removed successfully."),
                Err(e) => println!("{}", e),
            }
        },

        "remove_by_title" => {
            if num_args == 0 {
                println!("Please provide a title");
                return;
            }
            match todo_manager.remove_by_title(args) {
                Ok(_) => println!("Todo removed successfully."),
                Err(e) => println!("{}", e),
            }
        },

        "remove_by_status" => {
            if num_args == 0 {
                println!("Please provide a status");
                return;
            }
            todo_manager.remove_by_status(args.parse().unwrap());
        },

        "toggle_by_id" => {
            if num_args == 0 {
                println!("Please provide an id");
                return;
            }
            match todo_manager.toggle_by_id(args.parse().unwrap()) {
                Ok(_) => println!("Todo toggled successfully."),
                Err(e) => println!("{}", e),
            }
        },

        "toggle_by_title" => {
            if num_args == 0 {
                println!("Please provide a title");
                return;
            }
            match todo_manager.toggle_by_title(args) {
                Ok(_) => println!("Todo toggled successfully."),
                Err(e) => println!("{}", e),
            }
        },

        "toggle_by_status" => {
            if num_args == 0 {
                println!("Please provide a status");
                return;
            }
            todo_manager.toggle_by_status(args.parse().unwrap());
        },

        "print_all" => todo_manager.print(),

        "print_by_status" => {
            if num_args == 0 {
                println!("Please provide a status");
                return;
            }
            todo_manager.print_by_status(args.parse().unwrap());
        },

        "print_by_id" => {
            if num_args == 0 {
                println!("Please provide an id");
                return;
            }
            if let Some(todo) = todo_manager.find_by_id(args.parse().unwrap()) {
                todo.print();
            } else {
                println!("Todo with ID {} not found.", args);
            }
        },

        "print_by_title" => {
            if num_args == 0 {
                println!("Please provide a title");
                return;
            }
            todo_manager.print_by_title(args);
        },

        "save_to_json" => {
            if num_args == 0 {
                println!("Please provide a filename");
                return;
            }
            todo_manager.save_to_json(args).unwrap();
        },

        "load_from_json" => {
            if num_args == 0 {
                println!("Please provide a filename");
                return;
            }
            todo_manager.load_from_json(args).unwrap();
        },

        "exit" => std::process::exit(0),
        "quit" => std::process::exit(0),

        "help" => {
            let arg_value = args.parse::<String>().unwrap();
            if num_args == 0 || arg_value == "all"{
                println!("Available commands:");
                println!("add <title>: Add a new todo with the given title (error if a todo with the same title already exists)");
                println!("remove_by_id <id>: Remove the todo with the given id (error if no todo with the given id exists)");
                println!("remove_by_title <title>: Remove the todo with the given title (error if no todo with the given title exists)");
                println!("remove_by_status <status>: Remove all todos with the given status (error if no todos with the given status exist)");
                println!("toggle_by_id <id>: Toggle the status of the todo with the given id (error if no todo with the given id exists)");
                println!("toggle_by_title <title>: Toggle the status of the todo with the given title (error if no todo with the given title exists)");
                println!("toggle_by_status <status>: Toggle the status of all todos with the given status (error if no todos with the given status exist)");
                println!("print_all: Print all todos (error if no todos exist)");
                println!("print_by_status <status>: Print all todos with the given status (error if no todos with the given status exist)");
                println!("print_by_id <id>: Print the todo with the given id (error if no todo with the given id exists)");
                println!("print_by_title <title>: Print the todo with the given title (error if no todo with the given title exists)");
                println!("save_to_json <filename>: Save todos to the given json file");
                println!("load_from_json <filename>: Load todos from the given json file");
                println!("help: Print this help message");
                println!("exit or quit: Exit the program");
            }
            if arg_value == "add" {
                println!("add <title>: Add a new todo with the given title (error if a todo with the same title already exists)");
            }
            if arg_value == "remove" {
                println!("remove_by_id <id>: Remove the todo with the given id (error if no todo with the given id exists)");
                println!("remove_by_title <title>: Remove the todo with the given title (error if no todo with the given title exists)");
                println!("remove_by_status <status>: Remove all todos with the given status (error if no todos with the given status exist)");
            }
            if arg_value == "toggle" {
                println!("toggle_by_id <id>: Toggle the status of the todo with the given id (error if no todo with the given id exists)");
                println!("toggle_by_title <title>: Toggle the status of the todo with the given title (error if no todo with the given title exists)");
                println!("toggle_by_status <status>: Toggle the status of all todos with the given status (error if no todos with the given status exist)");
            }
            if arg_value == "print" {
                println!("print_all: Print all todos (error if no todos exist)");
                println!("print_by_status <status>: Print all todos with the given status (error if no todos with the given status exist)");
                println!("print_by_id <id>: Print the todo with the given id (error if no todo with the given id exists)");
                println!("print_by_title <title>: Print the todo with the given title (error if no todo with the given title exists)");
            }
            if arg_value == "save" {
                println!("save_to_json <filename>: Save todos to the given json file");
            }
            if arg_value == "load" {
                println!("load_from_json <filename>: Load todos from the given json file");
            }
            if arg_value == "json" {
                println!("save_to_json <filename>: Save todos to the given json file");
                println!("load_from_json <filename>: Load todos from the given json file");
            }
            if arg_value == "exit" || arg_value == "quit" {
                println!("exit or quit: Exit the program");
            }
            else {
                println!("Invalid argument: {}", arg_value);
            }
        }

        _ => println!("Invalid command"),

    }
}

fn main() {
    let mut todo_manager = TodoManager::new();
    loop {
        let input = take_input("> ");
        let (cmd, args) = parse_input_to_cmd(&input);
        interpret_cmd(cmd, args, &mut todo_manager);
    }
}

#[cfg(test)]
mod tests;

