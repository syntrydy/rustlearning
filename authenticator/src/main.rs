use std::io::{self, Write};

enum AuthState {
    LoggedOut,
    EnterringName { name: String },
    Login { user: String },
}

fn prompt(label: &str) -> Result<String, io::Error> {
    println!("{label}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    let mut state = AuthState::LoggedOut;
    loop {
        match &mut state {
            AuthState::LoggedOut => {
                println!("Commands: Login or Quit");
                let cmd = prompt("> ").unwrap_or_default();
                match cmd.as_str() {
                    "Login" => {
                        state = AuthState::EnterringName {
                            name: String::new(),
                        }
                    }
                    "Quit" => {
                        break;
                    }
                    _ => {
                        println!("Try Login or Quit")
                    }
                }
            }
            AuthState::EnterringName { name } => {
                if name.is_empty() {
                    *name = prompt("Enter your name").unwrap_or_default()
                } else {
                    println!("Welcome back to {name}.")
                }
            }
            AuthState::Login { user } =>{
                println!("Welcome to you {user}");
            }
            _ => {}
        }
    }
}
