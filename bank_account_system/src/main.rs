

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be greater than zero.".to_string());
        }
        self.balance += amount;
        Ok(())
    }
 
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("WithDraw amount must be greater than zero.".to_string());
        }
        self.balance -= amount;
        Ok(())
    }
    
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: String::from("Beyzanur "),
        balance: 2000.0,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: String::from("Emine Zeynep"),
        balance: 500.0,
    };
    match account1.deposit(1500.0) {
        Ok(_) => println!("Deposit successful. New balance: {}", account1.balance()),
        Err(e) => println!("Deposit failed: {}", e),
    }

    match account2.withdraw(200.0) {
        Ok(_) => println!("Withdrawal successful. New balance: {}", account2.balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    match account1.deposit(-100.0) {
        Ok(_) => println!("Deposit successful. New balance: {}", account1.balance()),
        Err(e) => println!("Deposit failed: {}", e),
    }

    match account2.withdraw(2000.0) {
        Ok(_) => println!("Withdrawal successful. New balance: {}", account2.balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    println!(
        "Merhaba {}  Bakiyeniz: {}",
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Merhaba {} Bakiyeniz: {}",
        account2.holder_name,
        account2.balance()
    );
}
