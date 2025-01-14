fn main() {
  let my_string:String = String::from("Hello");
  takes_ownership(my_string.clone());
  println!("{}" , my_string);  
}

fn takes_ownership(some_string :String) {
    println!("{}" , some_string);
}