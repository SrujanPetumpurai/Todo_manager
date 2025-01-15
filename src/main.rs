use std::fs::{self,OpenOptions};
use std::io::{self,BufRead,BufReader,Write};

const  FILE_NAME:&str = "./todo.txt";

fn view_tasks(){
    let file = OpenOptions::new().read(true).write(true).create(true).open(FILE_NAME);
    match file{
        Ok(file)=>{
            let reader = BufReader::new(file);
        let mut task_exist = false;
            for (index,line) in reader.lines().enumerate(){
                match line{
                    Ok(line)=>{
                        task_exist = true;
                        println!("{}:{}",index,line)
                    },
                    Err(_)=> println!("Coudln't read the file"),
                    
                }
            }
            if !task_exist{
                println!("No tasks found!")
            }
        }
        Err(e)=>{
            println!("Error is:{}",e)
        }
    }
  }
fn remove_task(){
    view_tasks();
    println!("Enter the index of the task you want to remove:");
    let mut choice = String::new(); 
    std::io::stdin().read_line(&mut choice).unwrap();
    let parse_choice = choice.trim().parse::<usize>();
    match parse_choice{
        Ok(task_number)=>{
            let mut file = OpenOptions::new().write(true).read(true).open(FILE_NAME).unwrap();
            let reader = BufReader::new(file);
            let tasks:Vec<String> =reader.lines().filter_map(|line| line.ok() ).collect(); 
            if task_number > tasks.len(){
                println!("Not right no.")
            }
            else{
                let updated_tasks:Vec<String> = tasks
                .into_iter()
                .enumerate()
                .filter(|(index,_)| *index !=task_number)
                .map(|(_,task)| task )
                .collect();

                fs::write(FILE_NAME,updated_tasks.join("\n")).unwrap();
                println!("Task removed!")
            }

        },
        Err(_)=>{
            println!("Format not correct")
        }

       }
}
fn add_task(){
    println!("Type the task you want to add!");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let mut file = OpenOptions::new().write(true).append(true).create(true).open(FILE_NAME).unwrap();
    let result =  writeln!(file,"{}",choice.trim());
    match result {
        Ok(())=>{
            println!("Task Added");
        },
        Err(e)=>{
            println!("Couldn't add task:{}",e)
        }
    }
}
fn main() {
    loop{
        println!("To-DO List Manager ");
        println!("1. View Tasks");
        println!("2. Add Task");
        println!("3. Remove Task");
        println!("4. Exit");
        println!("Choose an option");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim(){
            "1"=> view_tasks(),
            "2"=>add_task(),
            "3"=>remove_task(),
            "4"=>{
                println!("Exiting...");
                break;
            }
            _=>println!("Invalid option. Please try again."),
        }
    }
}

