/*This is a Todo_List CLI Application */
//import all the necessary
use std::io::{self, Write};
use std::collections::VecDeque;


//create the struct for the task
#[derive(Debug)]
struct Task{
    description: String,
    completed: bool,
    priority: u32,
}

//impl the struct
impl Task{
    //Create a new task using Associate Function
    fn create(description: String, priority: u32) -> Self {
        Self { description, completed: false, priority}
    }

    //Mark task as complete
    fn complete(&mut self){
        self.completed = true;
    }
}


//Create a struct to manage the task
/*Note the VecDeque is a collection used to store data just take it as your VEC in rust */
struct TaskList{
    tasks: VecDeque<Task>
}

//Implement TaskList
impl TaskList {
    //Associate Function here
    fn new() -> Self{
        Self { tasks: VecDeque::new(), }
    }

    //Methods for the struct here
    fn add_task(&mut self, description: String, priority: u32) {
        let task = Task::create(description, priority);
        self.tasks.push_back(task);
    }

    fn update_task(&mut self, new_description: String, new_priority: u32, index: usize) {
        if let Some(task) = self.tasks.get_mut(index){
            task.description = new_description;
            task.priority = new_priority;
        };
        
    }

    fn list_tasks(&self){
        if self.tasks.is_empty() {
            println!("List is empty");
        }
        else {
            for (index, task) in self.tasks.iter().enumerate(){
                let status = if task.completed {"Done"} else {"pending"};
                println!("{}: {} (Priority is {}) [{}]", index + 1, task.description, task.priority, status);
            }
        }
    }

    fn delete_task(&mut self, index: usize){
        if let Some(task) =  self.tasks.get_mut(index){
            self.tasks.remove(index);
            
        }
    }

    fn complete_task(&mut self, index: usize){
        if let Some(task) =  self.tasks.get_mut(index){
            task.complete();
            println!("Task '{}' completed", task.description);
            
        }else {
            println!("Invalid task number.");
        }
    }
}
fn main() {
    let mut todo_list = TaskList::new();

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Complete a task");
        println!("4. Delete a task");
        println!("5. Update a task");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                // Add a new task
                println!("Enter the task description: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                let description = description.trim().to_string();
                //Add the priority also
                println!("Enter the priority(Out of 10): ");
                let mut priority = String::new();
                io::stdin().read_line(&mut priority).expect("Failed to read input");
                let priority: u32 = priority.trim().parse().expect("Enter priority");
                todo_list.add_task(description, priority);
                println!("Task added.");
            }
            2 => {
                // List all tasks
                todo_list.list_tasks();
            }
            3 => {
                // Complete a task
                todo_list.list_tasks();
                println!("Enter the task number to complete: ");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).expect("Failed to read input");
                let task_num: usize = match task_num.trim().parse::<usize>() {
                    Ok(num) => num - 1, // Convert to zero-based index
                    Err(_) => {
                        println!("Invalid task number.");
                        continue;
                    }
                };
                todo_list.complete_task(task_num);
            }
            4 => {
                println!("Enter the task number to be deleted: ");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).expect("Failed to read input");
                let task_num: usize = match task_num.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Invalid task number.");
                        continue;
                    }
                };
                todo_list.delete_task(task_num);
            }
            5 => {
                println!("Enter the task number to updated: ");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).expect("Failed to read input");
                let task_num: usize = match task_num.trim().parse::<usize>() {
                    Ok(num) => num - 1, // Convert to zero-based index
                    Err(_) => {
                        println!("Invalid task number.");
                        continue;
                    }
                };
                println!("Enter the task description to be updated: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                let description = description.trim().to_string();
                //Add the priority also
                println!("Enter the priority to be updated(Out of 10): ");
                let mut priority = String::new();
                io::stdin().read_line(&mut priority).expect("Failed to read input");
                let priority: u32 = priority.trim().parse().expect("Enter priority");
                todo_list.update_task(description, priority, task_num);
                println!("Task Updated.");
            }
            6 => {
                // Exit the program
                println!("Goodbye!\nMake sure to do all your tasks");
                break;
            }
            _ => println!("Invalid choice, please enter a number between 1 and 6."),
        }
    }
}


