
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}


struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be positive".to_string())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else if amount <= 0.0 {
            Err("Withdrawal amount must be positive".to_string())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: "12345".to_string(),
        holder_name: "Alice".to_string(),
        balance: 0.0,
    };

    let mut account2 = BankAccount {
        account_number: "67890".to_string(),
        holder_name: "Bob".to_string(),
        balance: 0.0,
    };

    match account1.deposit(1000.0) {
        Ok(_) => println!("Deposit successful"),
        Err(e) => println!("Error: {}", e),
    }

    match account2.withdraw(700.0) {
        Ok(_) => println!("Withdrawal successful"),
        Err(e) => println!("Error: {}", e),
    }

    println!("Account 1 balance: {}", account1.balance());
    println!("Account 2 balance: {}", account2.balance());
}
