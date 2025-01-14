// String length in Rust

fn main(){
    let name = String::from("Helloasndadna Good Evening");
    let len = get_str_len(name );
    println!("The length of stirng is {}", len);
}

fn get_str_len(str :String) -> usize{
    str.chars().count()
    // This str is an arguments and which take string of any positive indexing size 
    // and as count function 
}