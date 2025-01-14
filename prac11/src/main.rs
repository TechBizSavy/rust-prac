/*
     Making a program of bank 
    function to display :- 
                account number,
                account holder,
                account status , 
                amount ,
                balance ,
                withdraw amount

                in RUST 
                let make it
*/


// enum for Account Status 



enum AccountStatus {
    Active,
    Frozen,
    Closed,
}


// Struct for Bank 
// bank consist some info 

struct Bank{
    account_number : String,
    account_holder  : String,
    balance : f64,
    status : AccountStatus,
}

impl Bank {
    // Making function to display all info 
    // &self is not but this keyword 
    fn display(&self) {
        let status_str = match &self.status {
            AccountStatus::Active => "Active",
            AccountStatus::Frozen => "Frozen",
            AccountStatus::Closed => "Closed",
        };
        println!("Account Number {} , Account Holder {} , Balance ${:.2} , Status {}",
                self.account_number , self.account_holder , self.balance , status_str
    );
    }   

    // The deposit func make that the account is active then
    // only u can deposit to it 
    fn deposit(&mut self , amount :f64) {
            if  let AccountStatus::Active = self.status {
                self.balance += amount;
                println!("Deposited ${:.2} , Now balance ${:.2}" , amount , self.balance);
            }else {
                println!("Account is closed ");
            }
    }


    // Now withdraw func which will 
    // check if the amount is less than can be withdrew
    fn withdrew(&mut self , amount :f64) {
        if let AccountStatus::Active = self.status {
            if amount<= self.balance {
                self.balance -= amount;
                println!("Withdrew ${:.2} , Balance ${:.2}" , amount , self.balance)
            }else {
                println!("Insuffiecnent funds")
            }
        }else {
            println!("Account is Closed")
        }
    
    }




}


// now making a the Bank object u can say 

fn  main() {
    let mut   account =  Bank{
        account_number : String::from("122132234"),
        account_holder : String::from("Sam"),
        balance : 5000.0,
        status : AccountStatus::Active,
    };

    account.display();
    println!("");
    account.deposit(5000.0);
    account.display();

    println!("");
    account.withdrew(2000.0);
    account.display();

}

