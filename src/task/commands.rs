use std::{io::{Error, Read}, process::exit};

use crate::util::io::{write_tasks_to_file, open_tasks_file};

use crate::task::todo::{TodoItem, TodoList};

pub fn add_new_task(todo: TodoItem) {
    let mut tasks: TodoList = get_all_tasks();

    // if item already exists, remove it.
    let is_updated = tasks.list.iter().find(|&e| e.id == todo.id);
    if let Some(item) = is_updated {
        let index = tasks.list.iter().position(|e| e.id == item.id).unwrap();
        tasks.list.remove(index);
    }
    tasks.list.push(todo);

    write_tasks_to_file(tasks);
}

pub fn mark_as_finished<'a>(task_id: u8) -> Result<&'a str, Error> {
    let mut tasks = get_all_tasks();
    // find todo item
    let t = tasks
        .list
        .iter()
        .find(|e| e.id == task_id)
        .expect("Todo item not found");
    // mark as finished
    let mut todo = TodoItem::duplicate(t);
    todo.completed = 'x';
    // remove item from array
    let index = tasks.list.iter().position(|e| e.id == t.id).unwrap();
    tasks.list.remove(index);
    // add item in same index
    tasks.list.insert(index, todo);
    write_tasks_to_file(tasks);
    Ok("")
}

pub fn delete_task(task_id: u8) {
    let mut tasks = get_all_tasks();
    let index= tasks.list.iter().position(|e| e.id == task_id).expect("Unable to locate Task");
    tasks.list.remove(index);
    write_tasks_to_file(tasks);
}

pub fn get_all_tasks() -> TodoList {
    let file_pointer = open_tasks_file(false);
    match file_pointer {
        Ok(mut file) => {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);
            serde_json::from_str(&contents)
                // in case of tasks.json error, return new
                .unwrap_or(TodoList::new())
        }
        Err(err) => {
            println!("Unable to read tasks.json");
            println!("{}", err);
            exit(-1)
        }
    }
}