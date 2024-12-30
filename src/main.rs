use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::time::Duration;
use chrono::{DateTime, Local, Utc};

struct Task {
    description: String,
    due_date: Option<DateTime<Utc>>,
}

impl Task {
    fn new(description: String, due_date: Option<DateTime<Utc>>) -> Self {
        Task { description, due_date }
    }

    fn is_due(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = Utc::now();
            now >= due_date
        } else {
            false
        }
    }
}

fn main() -> io::Result<()> {
    let mut tasks: Vec<Task> = Vec::new();

    // Load existing tasks
    load_tasks(&mut tasks)?;

    loop {
        println!("Todo List Manager");
        println!("1. Add a new task");
        println!("2. View all tasks");
        println!("3. Export tasks to CSV");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::Write::flush(&mut io::stdout())?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => add_task(&mut tasks)?,
            "2" => view_tasks(&tasks),
            "3" => export_tasks_to_csv(&tasks)?,
            "4" => {
                save_tasks(&tasks)?;
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }

        // Check for due tasks
        for task in &tasks {
            if task.is_due() {
                println!("Reminder: Task '{}' is due!", task.description);
            }
        }

        // Sleep for a short duration to avoid busy-waiting
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn add_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    print!("Enter task description: ");
    io::Write::flush(&mut io::stdout())?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = description.trim().to_string();

    loop {
        print!("Enter due date (optional, format YYYY-MM-DD HH:MM:SS) or leave blank for no due date: ");
        io::Write::flush(&mut io::stdout())?;
        let mut due_date_input = String::new();
        io::stdin().read_line(&mut due_date_input)?;
        let due_date_input = due_date_input.trim();

        if due_date_input.is_empty() {
            tasks.push(Task::new(description, None));
            break;
        } else {
            match DateTime::parse_from_str(due_date_input, "%Y-%m-%d %H:%M:%S").map(|dt| dt.with_timezone(&Utc)) {
                Ok(date) => {
                    tasks.push(Task::new(description, Some(date)));
                    break;
                }
                Err(e) => {
                    println!("Invalid date format. Please try again. Error: {}", e);
                }
            }
        }
    }

    Ok(())
}


fn view_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("Task {}: {}", i + 1, task.description);
        if let Some(due_date) = task.due_date {
            println!("Due date: {}", due_date.with_timezone(&Local));
        } else {
            println!("No due date");
        }
    }
}

fn load_tasks(tasks: &mut Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new().read(true).open("tasks.csv");

    if let Ok(file) = file {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    let description = parts[0].to_string();
                    let due_date = match parts[1].parse::<DateTime<Utc>>() {
                        Ok(date) => Some(date),
                        Err(_) => None,
                    };
                    tasks.push(Task::new(description, due_date));
                }
            }
        }
    }

    Ok(())
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("tasks.csv")?;

    for task in tasks {
        if let Some(due_date) = task.due_date {
            writeln!(file, "{},{}", task.description, due_date)?;
        } else {
            writeln!(file, "{}", task.description)?;
        }
    }

    Ok(())
}

fn export_tasks_to_csv(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("exported_tasks.csv")?;

    writeln!(file, "Description,Due Date")?;
    for task in tasks {
        if let Some(due_date) = task.due_date {
            writeln!(file, "{},{}", task.description, due_date)?;
        } else {
            writeln!(file, "{},", task.description)?;
        }
    }

    println!("Tasks have been exported to exported_tasks.csv");
    Ok(())
}
