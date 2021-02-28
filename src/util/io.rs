use std::{fs::{File, OpenOptions}, io::{Error, Write}, path::Path};

use crate::task::todo::TodoList;

pub fn write_tasks_to_file(todo_list: TodoList) {
    // re-serialize it to string
    let out = serde_json::to_string(&todo_list).unwrap();
    // println!("{}", out);

    // write to disk
    let out_file_pointer = open_tasks_file(true);
    match out_file_pointer {
        Ok(mut out_file) => {
            out_file.write_all(&out.into_bytes()).expect("Unable to write to disk.")
        }
        Err(err) => {
            println!("{}", err);
            println!("Unable to write to tasks.json");
        }
    }
}

pub fn open_tasks_file(write_only: bool) -> Result<File, Error> {
    let file_path = Path::new("./data/tasks.json");
    if write_only {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
        } else {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
    }
}
