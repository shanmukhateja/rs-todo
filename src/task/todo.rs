use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: u8,
    pub name: String,
    pub completed: char
}

impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        return  TodoItem {
            id: rand::random(),
            name: name.trim().to_string(),
            completed: ' '
        };
    }

    pub fn duplicate(t: &TodoItem) -> TodoItem {
        return TodoItem {
            id: t.id,
            name: t.name.clone(),
            completed: t.completed
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub list: Vec<TodoItem>
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            list: vec![]
        }
    }
}
