use std::io;

struct BankAccount {
    holder: String,
    balance: f64,
    number: String,
}

impl BankAccount {
    fn _new(holder: String, number: String) -> BankAccount {
        BankAccount {
            holder: holder,
            balance: 0.0,
            number: number,
        }
    }
    fn new_with_balance(holder: String, balance: f64, number: String) -> BankAccount {
        if balance < 0.0 {
            return BankAccount {
                holder: holder,
                balance: 0.0,
                number: number,
            };
        }
        BankAccount {
            holder: holder,
            balance: balance,
            number: number,
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<bool, String> {
        let result = match amount {
            x if x < 0.0 => Err("The amount should not be negative.".to_string()),
            x if x < 10.0 && x >= 0.0 => Err("The minimum amount for deposit is 10.0".to_string()),
            _ => {
                self.balance += amount;
                Ok(true)
            }
        };
        result
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!("{amount} has be deposit to your account successfully!")
            } else {
                println!(
                    "Insufficient balance: your current balance is {:.2}",
                    &self.balance
                )
            }
        } else {
            println!("Impossible to withdraw negative amount({})!", amount)
        }
    }
    fn check_balance(&self) {
        println!("Current balance: ${:.2}", self.balance);
    }

    fn display_account_info(&self) {
        println!("\n--- Account Information ---");
        println!("Account Holder: {}", self.holder);
        println!("Account Number: {}", self.number);
        println!("Current Balance: ${:.2}", self.balance);
        println!("---------------------------");
    }
}

fn get_user_input() -> String {
    let mut user_input: String = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    let user_input = user_input.trim().to_string();
    user_input
}

fn get_amount() -> f64 {
    get_user_input().parse::<f64>().unwrap_or_default()
}

fn show_menu() {
    println!("What would you like to do?");
    println!("1. Check Balance");
    println!("2. Make Deposit");
    println!("3. Make Withdrawal");
    println!("4. View Account Info");
    println!("5. Exit");
    println!("Enter your choice (1-5):");
}

fn main() {
    println!("Welcome to Simple Bank!");
    println!("=======================");
    println!("Let's create your bank account");
    println!("What is your first name?");
    let first_name = get_user_input();
    println!("What is your initial balance?");
    let balance = get_amount();
    let mut bank_account =
        BankAccount::new_with_balance(first_name, balance, "AccountN01".to_string());
    bank_account.display_account_info();
    loop {
        show_menu();
        let choice = get_user_input();
        match choice.as_str() {
            "1" => {
                bank_account.check_balance();
            }
            "2" => {
                println!("Enter the amount to deposit");
                let sum = get_amount();
                let result = bank_account.deposit(sum);
                match result {
                    Ok(_x) => {
                        println!("{} has been deposit to your account successfuly!", sum)
                    }
                    Err(message) => {
                        println!("{}", message)
                    }
                }
            }
            "3" => {
                println!("Enter the amount to withdraw");
                let sum = get_amount();
                bank_account.withdraw(sum);
            }
            "4" => {
                bank_account.display_account_info();
            }
            "5" => {
                break;
            }
            _ => {}
        }
    }
}
