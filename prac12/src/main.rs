// Function in Rust
// Lets make a simple Sum of 
// 2 numbers 


// always remeber their needs to 
// main function




fn main(){
  
    println!("Calling the sum func");

    // ans here func calling
    sum_calculator(200,400  );

}

// here i32 is signed number positive of 32 bits

// also this is called as func defining
fn sum_calculator(num1 : i32 , num2 : i32){
    let result = num1 + num2;
    println!("The sum of {} and {} is {}" ,
     num1 , num2 , result);
}



// making a sum_calculator func
// and defining with some parameters
// of num1 and num2 