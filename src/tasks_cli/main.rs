

struct Task {
    id: u32,
    description: String,
    done: bool,
}
use std::io;
use colored::*;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("{}", "What do you want to do?".green().bold());
        println!("{}", "1. Add a task".cyan());
        println!("{}", "2. Mark a task as done".cyan());
        println!("{}", "3. List all tasks".cyan());
        println!("{}", "4. Quit".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&mut tasks),
            2 => mark_done(&mut tasks),
            3 => list_tasks(&tasks),
            4 => break,
            _ => continue,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("{}", "Enter your task description:".blue());

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    let task = Task {
        id: tasks.len() as u32 + 1,
        description: description.trim().to_string(),
        done: false,
    };

    tasks.push(task);
    println!("Task added!");
}

fn mark_done(tasks: &mut Vec<Task>) {
    println!("{}", "Enter the task id to mark as done:".blue());

    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: u32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        },
    };

    for task in tasks.iter_mut() {
        if task.id == id {
            task.done = true;
            println!("Task marked as done!");
            return;
        }
    }
    println!("Task not found.");
}

fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        let status = if task.done {
            "Done".green()
        } else {
            "Not done".red()
        };
        println!("{}: {} - {}", task.id, task.description, status);
    }
}