use std::{env, process::exit};

use task::{commands::{add_new_task, delete_task, get_all_tasks, mark_as_finished}, todo::TodoItem};
use util::cli::{show_usage_and_exit, read_user_input};

mod task;
mod util;

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