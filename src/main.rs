use std::{env, io::stdin, process::exit};

mod data;
use data::{add_new_task, delete_task, get_all_tasks, mark_as_finished};

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    id: u8,
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return  TodoItem {
            id: rand::random(),
            name: name.trim().to_string(),
            completed: ' '
        };
    }

    fn duplicate(t: &TodoItem) -> TodoItem {
        return TodoItem {
            id: t.id,
            name: t.name.clone(),
            completed: t.completed
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            list: vec![]
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_usage_and_exit();
    }
    let command = &args[1];

    run(command);
}

fn run(command: &str) {
    let todo_list = get_all_tasks();
    match command {
        "list" => {
            if todo_list.list.len() == 0 {
                println!("No tasks found");
                exit(-2);
            }
            println!("ID \t Task Name \t Status");
            for item in &todo_list.list {
                println!("{} \t {} \t [{}]",item.id, item.name, item.completed);
            }
        }
        "add" => {
            let buf = read_user_input("Enter task name:");
            let todo = TodoItem::new(buf);
            add_new_task(todo);
            println!("Addded successfully");
        }
        "delete" => {
            println!("Listing all tasks, enter task ID:");
            run("list");
            let task_id = read_user_input("Enter task name:");
            let int_task_id = task_id.parse::<u8>().expect("Invalid task ID provided.");
            delete_task(int_task_id);
        }
        "finished" => {
            if todo_list.list.len() == 0 {
                println!("No tasks found");
                exit(-2);
            }
            println!("Listing all tasks, enter task ID:");
            run("list");
            let task_id = read_user_input(">> ").parse::<u8>().unwrap();
            match mark_as_finished(task_id) {
                Ok(_) => {
                    println!("Task has been marked as complete");
                }
                Err(e) => {
                    println!("Unable to mark task as complete.");
                    println!("{}", e);
                }
            }
        }
        _ => {
            // default case
            println!("Invalid command '{}'", command);
            show_usage_and_exit();
        }
    }
}

fn read_user_input(input_msg: &str) -> String {
    println!("{}", input_msg);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    return buf.trim().to_string();
}

fn show_usage_and_exit() {
    println!("Usage: rs-todo list|add|finished <args>");
    exit(-1);
}