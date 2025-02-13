use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "todo_list.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: VecDeque<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: VecDeque::new() }
    }

    fn load() -> Self {
        let mut file = match File::open(FILE_PATH) {
            Ok(file) => file,
            Err(_) => return Self::new(),
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap_or(0);

        serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn add_task(&mut self, title: String) {
        let id = self.tasks.len() + 1;
        self.tasks.push_back(Task { id, title, done: false });
        self.save();
        println!("✅ Task added successfully!");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks found!");
        } else {
            println!("📌 To-Do List:");
            for task in &self.tasks {
                let status = if task.done { "✔️ Done" } else { "❌ Pending" };
                println!("{}. [{}] {}", task.id, status, task.title);
            }
        }
    }

    fn mark_done(&mut self, task_id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.done = true;
            self.save();
            println!("✅ Task marked as done!");
        } else {
            println!("⚠️ Task not found!");
        }
    }

    fn remove_task(&mut self, task_id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id) {
            self.tasks.remove(pos);
            self.save();
            println!("🗑️ Task removed successfully!");
        } else {
            println!("⚠️ Task not found!");
        }
    }
}

fn main() {
    let mut todo_list = TodoList::load();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("❌ Invalid usage. Available commands:");
        println!("  add \"Task Description\"   - Add a new task");
        println!("  list                     - List all tasks");
        println!("  done <task_id>           - Mark a task as done");
        println!("  remove <task_id>         - Remove a task");
        return;
    }

    match args[1].as_str() {
        "add" if args.len() >= 3 => {
            let title = args[2..].join(" ");
            todo_list.add_task(title);
        }
        "list" => {
            todo_list.list_tasks();
        }
        "done" if args.len() == 3 => {
            if let Ok(task_id) = args[2].parse::<usize>() {
                todo_list.mark_done(task_id);
            } else {
                println!("⚠️ Invalid task ID");
            }
        }
        "remove" if args.len() == 3 => {
            if let Ok(task_id) = args[2].parse::<usize>() {
                todo_list.remove_task(task_id);
            } else {
                println!("⚠️ Invalid task ID");
            }
        }
        _ => {
            println!("❌ Unknown command. Use 'list', 'add', 'done <id>', or 'remove <id>'");
        }
    }
}
