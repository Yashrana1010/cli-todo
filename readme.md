# To Do List

The CLI To-Do List is a command-line tool that allows users to manage their tasks efficiently. This application provides functionality to add, list, mark as completed, and remove tasks from a to-do list. It stores tasks locally in a JSON file, ensuring that your data persists across sessions.

## The tool offers an intuitive interface and several useful features:

- Add tasks with descriptions
- List tasks with filtering options (completed, pending)
- Mark tasks as completed
- Remove tasks by ID
- Task status tracking (Pending/Completed)
- Duration tracking for tasks that are still in progress
- Built with Rust, this application leverages popular libraries like clap for argument parsing, serde for serialization, 
- hrono for date handling, and colored for output formatting, giving it a clean and user-friendly interface.


## How to Run the Project 

Clone the repository using :

``` git Clone https://github.com/Yashrana1010/cli-todo ```

Navigate to the project directory and run the following command to build the application:

``` cargo build ```

After Building the Project Run the Below Command to start the project 

``` Cargo run  ```



##  Commands to use the To-Do list

To `add` the item in To-do List Run the Below Command

```  cargo run -- add "buy grocery" ```

After running the command you will get Output something like this 

``` Task added successfully! ``` 

To `List` the item in To-do List Run the Below Command

``` cargo run -- list ```

After running the command you will get Output something like this 


 ``` === Tasks ===
==================================================

✓ Task #1
├─ Description: I have to buy grocery
├─ Status: Completed
├─ Created: 2024-11-14 12:58:28
└─ Completed: 2024-11-14 13:01:36
--------------------------------------------------

✓ Task #2
├─ Description: I have to Wash clothes
├─ Status: Completed
├─ Created: 2024-11-14 12:59:09
└─ Completed: 2024-11-14 12:59:57
--------------------------------------------------

○ Task #3
├─ Description: go home
├─ Status: Pending
├─ Created: 2024-11-14 13:34:00
└─ Duration: 1 hours, 54 minutes
--------------------------------------------------

○ Task #4
├─ Description: buy grocery
├─ Status: Pending
├─ Created: 2024-11-14 15:24:36
└─ Duration: 3 minutes
--------------------------------------------------

=== Summary ===
Total tasks: 4
Completed: 2
Pending: 2
================================================== 
```


To ` Done ` the item in To-do List Run the Below Command

``` cargo run -- done 1 ```

After running the command you will get Output something like this. You can see the checked mark will be updated and status will show completed

``` 
=== Tasks ===
==================================================

✓ Task #1
├─ Description: I have to buy grocery
├─ Status: Completed
├─ Created: 2024-11-14 12:58:28
└─ Completed: 2024-11-14 13:01:36
--------------------------------------------------

``` 


To `remove` the item in To-do List Run the Below Command

``` cargo run -- remove 4 ```

After running the command you will get Output something like this 

``` Task #4 removed! ``` 

## For Windows 

If you want to use this project in your Windows System you can download the Exe file from the Repo

- After Downloading the Exe file 

- Navigate the file directory and Open that File location in terminal 

- run the below command 

  ``` ./todolist.exe -h ```

- You will see something like this 

```
Usage: todolist.exe <COMMAND>

Commands:
  add     Add a new task
  list    List all tasks
  done    Mark a task as completed
  remove  Remove a task
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- Go through the above help command To use the Application .
