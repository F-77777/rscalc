use colored::Colorize;
use std::io as lmao;
use std::io::Write;

fn main() {
    let err_color_reg = (145, 0, 36);
    let err_color_light = (255, 0, 63);

    print!("{}", "This is a calculator program".blue().bold());

    loop {
        println!("\n\n{}", "1. Addition".yellow());
        println!("{}", "2. Subtraction".yellow());
        println!("{}", "3. Multiplication".yellow());
        println!("{}", "4. Division".yellow());
        println!("{}\n", "5. Exit".yellow());

        print!("{}", "Enter an operation: ".blue().bold());
        lmao::stdout().flush().unwrap();

        let mut choice = String::new();
        lmao::stdin()
            .read_line(&mut choice)
            .expect("There was an error reading input!");

        match choice.trim() {
            "5" => {
                println!("{}", "Farewell!".green().bold());
                std::process::exit(0);
            }
            "1" | "2" | "3" | "4" => {
            }
            _ => {
                println!(
                    "{}\n{}",
                    "Invalid choice!".red().bold(),
                    "Please enter 1, 2, 3, 4, or 5."
                );
                continue;
            }
        }

        let mut x = String::new();
        let mut y = String::new();

        print!("Enter the first number: ");
        lmao::stdout().flush().unwrap();
        lmao::stdin()
            .read_line(&mut x)
            .expect("There was an error reading input!");

        let x: i64 = match x.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "There has been an error:"
                        .truecolor(err_color_reg.0, err_color_reg.1, err_color_reg.2)
                        .bold(),
                    format!("{e}")
                        .truecolor(err_color_light.0, err_color_light.1, err_color_light.2)
                        .bold()
                );
                continue;
            }
        };

        print!("Enter the second number: ");
        lmao::stdout().flush().unwrap();
        lmao::stdin()
            .read_line(&mut y)
            .expect("There was an error reading input!");

        let y: i64 = match y.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "There has been an error:"
                        .truecolor(err_color_reg.0, err_color_reg.1, err_color_reg.2)
                        .bold(),
                    format!("{e}")
                        .truecolor(err_color_light.0, err_color_light.1, err_color_light.2)
                        .bold()
                );
                continue;
            }
        };

        match choice.trim() {
            "1" => println!(
                "{}{}\n",
                "The sum is: ".blue().bold(),
                (x + y).to_string().bright_blue().bold()
            ),
            "2" => println!(
                "{}{}\n",
                "The difference is: ".blue().bold(),
                (x - y).to_string().bright_blue().bold()
            ),
            "3" => println!(
                "{}{}\n",
                "The product is: ".blue().bold(),
                (x * y).to_string().bright_blue().bold()
            ),
            "4" => {
                if y == 0 {
                    println!("{}", "Error: Division by zero is not allowed!".red().bold());
                } else {
                    println!(
                        "{}{}\n",
                        "The quotient is: ".blue().bold(),
                        (x / y).to_string().bright_blue().bold()
                    );
                }
            }
            _ => {}
        }
    }
}
