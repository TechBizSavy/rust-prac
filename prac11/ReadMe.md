// Define an enum for AccountStatus
enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

// Define a struct for BankAccount
struct BankAccount {
    account_number: String,
    account_holder: String,
    balance: f64,
    status: AccountStatus,
}

impl BankAccount {
    // Function to display account details
    fn display(&self) {
        let status_str = match &self.status {
            AccountStatus::Active => "Active",
            AccountStatus::Frozen => "Frozen",
            AccountStatus::Closed => "Closed",
        };
        println!(
            "Account Number: {}, Account Holder: {}, Balance: ${:.2}, Status: {}",
            self.account_number, self.account_holder, self.balance, status_str
        );
    }

    // Function to deposit money
    fn deposit(&mut self, amount: f64) {
        if let AccountStatus::Active = self.status {
            self.balance += amount;
            println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
        } else {
            println!("Cannot deposit. Account is not active.");
        }
    }

    // Function to withdraw money
    fn withdraw(&mut self, amount: f64) {
        if let AccountStatus::Active = self.status {
            if amount <= self.balance {
                self.balance -= amount;
                println!("Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);
            } else {
                println!("Insufficient balance.");
            }
        } else {
            println!("Cannot withdraw. Account is not active.");
        }
    }
}

fn main() {
    // Create a bank account
    let mut account = BankAccount {
        account_number: String::from("123456789"),
        account_holder: String::from("John Doe"),
        balance: 1000.0,
        status: AccountStatus::Active,
    };

    // Display account details
    account.display();

    // Deposit and withdraw money
    account.deposit(200.0);
    account.withdraw(150.0);
    
    // Change account status to Frozen
    account.status = AccountStatus::Frozen;
    account.withdraw(50.0); // This should not allow withdrawal
    
    // Display account details again
    account.display();
}
