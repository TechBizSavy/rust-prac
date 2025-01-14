// Find first letter a 
/*
    will make one function which takes string and in cheks 
    if a is there and if yes then return the index of it 


    and in this video  we will learn about Option in rust
    they are nothing a enum which helps or provide us None 
    and Some task to do

    ex:-

    enum Option{
        Some(i32),
        None
    }
 */

//  fn main(){
//     let index= find_first_a(String::from("papa"));
//     // Making a match case 
//     match index {
//         Some(value) => println!("{}" ,value),
//         None => println!("a not found ")
//     }
//  }


//  fn find_first_a(s :String)->Option<i32>{

//     // We made a loop that takes string and returns index and 
//     // will check our string 
//     for (index , char) in s.chars().enumerate()  {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None; 
//  }



/*
    In this video we are making reading a file 
    By using internal function or keywords


 */

use std::fs::read_to_string;


fn main(){
    let ans = find_file("a.txt");
    match ans {
        Ok(d) => println!("{}" ,d ),
        Err(e) => println!("{}" ,e),
    }
    
}


// We are making a function in which i am taking a string 
// which has file name in it 

fn find_file(file_path: &str) -> Result<String , String>{
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("Content not found "))
    }
}