## Todo Manager: A Command-Line Todo List Application

This is a command-line todo list application written in Rust. It allows you to create, manage, and view your todos.

### Features

* Add new todos with titles.
* Remove todos by ID, title, or status (completed/incomplete).
* Toggle the completion status of a todo.
* Find a todo by ID or title.
* Print all todos, or filter by status.
* Save and load todos from JSON files.

### Usage

The application is run from the command line.  Here are some basic commands:

```
add <title>        : Add a new todo with the given title.
remove_by_id <id>   : Remove the todo with the given ID.
remove_by_title <title>: Remove the todo with the given title.
remove_by_status <status>: Remove all todos with the given status (true: completed, false: incomplete).
toggle_by_id <id>   : Toggle the status of the todo with the given ID.
toggle_by_title <title>: Toggle the status of the todo with the given title.
toggle_by_status <status>: Toggle the status of all todos with the given status.
print_all          : Print all todos.
print_by_status <status>: Print all todos with the given status.
print_by_id <id>     : Print the todo with the given ID.
print_by_title <title>: Print the todo with the given title.
save_to_json <filename>: Save todos to the given JSON file.
load_from_json <filename>: Load todos from the given JSON file.
help                : Print this help message.
exit|quit           : Exit the program.
```

For detailed information about each command and its arguments, run `help` or `help <command>`.

### Example Usage

Add a new todo:

```
> add Buy groceries
```

List all todos:

```
> print_all
ID: 0, Title: Buy groceries, Status: Incomplete
```

Mark the todo as completed:

```
> toggle_by_id 0
```

Print all completed todos:

```
> print_by_status true
ID: 0, Title: Buy groceries, Status: Completed
```

Save todos to a file:

```
> save_to_json todos.json
```

Exit the program:

```
> exit
```

### Dependencies

This project requires Rust and the following crates:

* `serde`: for JSON serialization and deserialization
* `serde_json`: for working with JSON data format
* `indexmap`: for efficient lookup of todo titles

To install Rust and manage dependencies, refer to the official Rust documentation: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Tests

The project includes unit tests for core functionalities. You can run the tests with the following command:

```
cargo test
```

### Contributing

Feel free to contribute to this project by creating pull requests. Make sure to include tests for any new features you add.
