// Fibonnaci series in rust 
// lets make it 

// first make a main func

fn main(){
    println!("The fib series is {}" , fib(5));
}

// making a separate func
// this func take a number which can't
// be negative and return the u32 means
// unsigned number 
fn fib(num : u32)-> u32 {
    // making the first and second number
    let mut first = 0;
    let mut second = 1;
    // as u might know to use the variable 
    // in for loop they must be mutable

    if num == 0 {
        return  0;
    }

    if num == 1 {
        return  1;
    }
// the for loop enumrate from 0 to n-1
// and the _ is defalt replaced for var
    for _ in 0..(num-1)  {

        // the logic is simple 
// we are making the temp as the sec 1 
// as soo its increment the value become 
// 1+1 = 2 and so 
        let temp = second;
        second = second + first;
        first = temp;
    }
    return  second;
}