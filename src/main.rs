use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use colored::*;
use directories::ProjectDirs;
use chrono::{DateTime, Local};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
// ------------------Commands to Use in Cli
#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task description
        #[arg(required = true)]
        description: Vec<String>,
    },
    /// List all tasks
    List {
        /// Show only completed tasks
        #[arg(long)]
        completed: bool,
        /// Show only pending tasks
        #[arg(long)]
        pending: bool,
    },
    /// Mark a task as completed
    Done {
        /// The task ID
        id: usize,
    },
    /// Remove a task
    Remove {
        /// The task ID
        id: usize,
    },
}

//------------Structure of the Todo List Task

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    #[serde(default = "Local::now")]
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

struct TodoList {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

impl TodoList {
    fn new() -> Self {
        let file_path = if let Some(proj_dirs) = ProjectDirs::from("com", "todo", "todo-cli") {
            let data_dir = proj_dirs.data_dir();
            fs::create_dir_all(data_dir).unwrap_or_else(|_| {
                println!("Failed to create data directory");
            });
            data_dir.join("tasks.json")
        } else {
            PathBuf::from("tasks.json")
        };

        let tasks = Self::load_tasks(&file_path).unwrap_or_default();
        TodoList { tasks, file_path }
    }

    fn load_tasks(file_path: &PathBuf) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                let tasks = serde_json::from_str(&contents)?;
                Ok(tasks)
            }
            Err(_) => Ok(Vec::new()),
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) {
        let id = if let Some(last_task) = self.tasks.last() {
            last_task.id + 1
        } else {
            1
        };

        let task = Task {
            id,
            description,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.tasks.push(task);
        self.save().unwrap_or_else(|_| {
            println!("Failed to save tasks");
        });
    }

    fn list_tasks(&self, completed_filter: Option<bool>) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks found.".yellow());
            return;
        }

        let filtered_tasks: Vec<&Task> = self.tasks
            .iter()
            .filter(|task| {
                if let Some(completed) = completed_filter {
                    task.completed == completed
                } else {
                    true
                }
            })
            .collect();

        if filtered_tasks.is_empty() {
            println!("{}", "No matching tasks found.".yellow());
            return;
        }

        println!("\n{}", "=== Tasks ===".bold());
        println!("{}", "=".repeat(50));

        for task in filtered_tasks {
            // Print task header with ID and status
            let status = if task.completed {
                "✓".green()
            } else {
                "○".red()
            };
            println!("\n{} Task #{}", status, task.id.to_string().cyan());
            
            // Print task details
            println!("├─ Description: {}", task.description);
            println!("├─ Status: {}", if task.completed {
                "Completed".green()
            } else {
                "Pending".yellow()
            });
            println!("├─ Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S").to_string().blue());
            
            if let Some(completed_at) = task.completed_at {
                println!("└─ Completed: {}", completed_at.format("%Y-%m-%d %H:%M:%S").to_string().green());
            } else {
                println!("└─ Duration: {}", format_duration(Local::now() - task.created_at).magenta());
            }
            
            println!("{}", "-".repeat(50));
        }

        // Print summary
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        println!("\n{}", "=== Summary ===".bold());
        println!("Total tasks: {}", total.to_string().cyan());
        println!("Completed: {}", completed.to_string().green());
        println!("Pending: {}", (total - completed).to_string().yellow());
        println!("{}", "=".repeat(50));
    }

    fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            task.completed_at = Some(Local::now());
            self.save().unwrap_or_else(|_| {
                println!("Failed to save tasks");
            });
            return true;
        }
        false
    }

    fn remove_task(&mut self, id: usize) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.save().unwrap_or_else(|_| {
                println!("Failed to save tasks");
            });
            return true;
        }
        false
    }
}

// Helper function to format duration
fn format_duration(duration: chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();
    let days = total_seconds / (24 * 3600);
    let hours = (total_seconds % (24 * 3600)) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours", days, hours)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();

    match cli.command {
        Commands::Add { description } => {
            let description = description.join(" ");
            todo_list.add_task(description);
            println!("{}", "Task added successfully!".green());
        }
        Commands::List { completed, pending } => {
            let filter = if completed {
                Some(true)
            } else if pending {
                Some(false)
            } else {
                None
            };
            todo_list.list_tasks(filter);
        }
        Commands::Done { id } => {
            if todo_list.complete_task(id) {
                println!("{} {}", "Task".green(), format!("#{} marked as completed!", id).green());
            } else {
                println!("{} {}", "Error:".red(), format!("Task #{} not found.", id).red());
            }
        }
        Commands::Remove { id } => {
            if todo_list.remove_task(id) {
                println!("{} {}", "Task".green(), format!("#{} removed!", id).green());
            } else {
                println!("{} {}", "Error:".red(), format!("Task #{} not found.", id).red());
            }
        }
    }
}