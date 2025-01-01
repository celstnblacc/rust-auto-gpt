use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = std::io::stdout();

        // Decide on the print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("{}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset the color
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    print!("> ");
    println!("{}", question);

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    // Read the user's response
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read user input");

    // Remove the newline character
    response.trim().to_string()
}

pub fn confirm_safe_code() -> bool {
    let mut stdout = stdout();
    loop {
        
        //Print the question  in specific color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        print!("");
        println!("You are about to run code written by an AI.");
        println!("Review the code before continue.");

        //Reset the color
        stdout.execute(ResetColor).unwrap();

        //Present the options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("1. Continue (Y/y)");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("2. Exit (N/n)");
        stdout.execute(ResetColor).unwrap();

        //Read the user's response
        let mut response = String::new();
        stdin()
            .read_line(&mut response)
            .expect("Failed to read user input");

        //Remove the newline character
        let response = response.trim().to_lowercase();

        match response.as_str() {
            "1" | "yes" | "y" | "continue" | "c" | "ok" => return true,
            "2" | "no" | "n" | "exit" | "e" => return false,
            _ => println!("Invalid input. Please enter 1 or 2."),
        }  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_message() {
        PrintCommand::AICall
            .print_agent_message("Managing agent", "Testing, processing, and executing");
    }
}
