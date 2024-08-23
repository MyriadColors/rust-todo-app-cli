use super::*;

#[test]
fn add_todo() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    assert_eq!(todo_manager.todos.len(), 1);
}

#[test]
fn remove_todo_by_id() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    let id = todo_manager.todos[0].id;
    todo_manager.remove_by_id(id).unwrap();
    assert_eq!(todo_manager.todos.len(), 0);
}

#[test]
fn toggle_todo_status() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    let id = todo_manager.todos[0].id;
    todo_manager.toggle_by_id(id).unwrap();
    assert!(todo_manager.todos[0].status);
}

#[test]
fn find_todo_by_title() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    let found_todo = todo_manager.find_by_title("Test Todo");
    assert!(found_todo.is_ok());
    assert_eq!(found_todo.unwrap().title, "Test Todo");
}

#[test]
fn save_and_load_to_json() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo 1".to_string()).unwrap();
    todo_manager.add("Test Todo 2".to_string()).unwrap();

    todo_manager.save_to_json("test.json").unwrap();

    let mut new_todo_manager = TodoManager::new();
    new_todo_manager.load_from_json("test.json").unwrap();

    assert_eq!(new_todo_manager.todos.len(), 2);
    assert_eq!(new_todo_manager.todos[0].title, "Test Todo 1");
    assert_eq!(new_todo_manager.todos[1].title, "Test Todo 2");
}

#[test]
fn find_todo_by_id() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    let found_todo = todo_manager.find_by_id(0);
    assert!(found_todo.is_some());
    assert_eq!(found_todo.unwrap().title, "Test Todo");
}

#[test]
fn remove_todo_by_title() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.remove_by_title("Test Todo").unwrap();
    assert_eq!(todo_manager.todos.len(), 0);
}

#[test]
fn toggle_todo_by_title() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.toggle_by_title("Test Todo").unwrap();
    assert!(todo_manager.todos[0].status);
}

#[test]
fn toggle_todo_by_status() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.toggle_by_status(true);
    assert!(todo_manager.todos[0].status);
}

#[test]
fn print_todo_by_id() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.print_by_id(0);
}

#[test]
fn print_todo_by_title() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.print_by_title("Test Todo");
}

#[test]
fn print_todo_by_status() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.print_by_status(true);
}

#[test]
fn print_all() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.print();
}

#[test]
fn remove_all() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.remove_all_todos();
    assert_eq!(todo_manager.todos.len(), 0);
}

#[test]
fn remove_by_id_range() {
    let mut todo_manager = TodoManager::new();
    todo_manager.add("Test Todo".to_string()).unwrap();
    todo_manager.remove_by_id_range(0, 1);
    assert_eq!(todo_manager.todos.len(), 0);
}

