use colored::Colorize;
use std::io;
use Command::*;
use std::io::Write;
use std::num::IntErrorKind;
enum Command {
    RemoveAt(usize),
    Push(i64),
}

fn main() {
    let mut list = vec![1];
    println!("{}", "Type 1 to Remove an element at a specific index, 2 to push a number, and 3 to exit".blue().bold());
    // Capture list by mutable reference to avoid ownership issues
    let clos = |comm: Command, list: &mut Vec<i64>| match comm {
        RemoveAt(x) => {
            if x >= list.len() {
                println!("{}", "Out of bounds".red().bold());
                std::process::exit(0);
            } else {
                list.remove(x);
            }
        }
        Push(a) => list.push(a),
    };
    loop {
        let mut choice = String::new();
        println!("\n{}{}", "The current contents are: ".blue().bold(),
         format!("{:?}", list).cyan().bold());
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("Invalid input\n Enter 1 to remove an element at an index, 2 to push a number, and 3 to exit");
        match choice.as_str().trim() {
            "1" => {
                if list == vec![] {
                    println!("{}", "The vector is empty!".red().bold());
                    println!("{}", "Please add a number to the vector first".red().bold());
                    continue;
                }
                print!("Enter index: ");
                io::stdout().flush().unwrap();
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line");
                let x: usize = match x.trim().parse() {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("\n{}", "There was an error".red().bold());
                        eprintln!("{} {}", "Error:".red().bold(), e);
                        continue;
                    },
                };
                if x > list.len() {
                    println!("{}", "There is not a number at that index!".red().bold());
                    continue;
                } else {
                    clos(RemoveAt(x - 1), &mut list);
                    continue;
                }
            }

            "2" => {
                let mut a = String::new();
                print!("\n{}", "Enter number to push: ".blue().bold());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut a).expect("Failed to read input");
                match a.trim().parse::<i64>() {
                    Ok(a) => {
                        clos(Push(a), &mut list);
                        println!("{} {} {}", "Successfully added".green().bold(), a.to_string().green().bold(), "to the vector!".green().bold());
                        continue;
                    }
                    Err(e) => {
                        eprintln!("There was an error");
                        if *e.kind() == IntErrorKind::NegOverflow || *e.kind() == IntErrorKind::PosOverflow {
                            eprintln!("Err: {}", e); 
                            eprintln!("Enter an integer from {} to {}", i64::MIN, i64::MAX);
                        } else {
                            eprintln!("Err: {}", e);
                        }
                        continue;
                    }
                };
            }
            "3" => {
                println!("{}", "Farewell!".green().bold());
                std::process::exit(0);
            }
            _ => {
                println!("{}", "You didn't enter a valid choice!".red().bold());
                continue;
            }
        };
    }
}
