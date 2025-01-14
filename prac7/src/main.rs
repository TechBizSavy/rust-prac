use chrono::{Local , Utc};

fn main(){

    let now = Utc::now();
    println!("Current date  and time in UTC {} " , now);

    let formatted = now.format("%Y-%M-%D %H:%M:%S");
    println!("The formatted date  {}" , formatted);

    let local = Local::now();
    println!("Current date and time in local {}" , local);

    
}