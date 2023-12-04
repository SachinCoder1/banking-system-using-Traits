
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}


struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}


impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds.");
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

    account1.deposit(1000.0);
    account2.deposit(500.0);

    account1.withdraw(200.0);
    account2.withdraw(700.0); // This should show "Insufficient funds."

    println!("Account 1 balance: {}", account1.balance());
    println!("Account 2 balance: {}", account2.balance());
}
