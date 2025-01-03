// 1
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}
//2
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    //3
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    //4
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
    //5
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    //6
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
    //7
    account1.deposit(1500.0);

    // 8
    account2.withdraw(200.0);

    //9
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
