// Structs in Rust
/*
    So if are coming from c++ lanuage prerequisits 
    you might then structs and their uses 

    So in rust also the structs are similar to c++
    and also we can implement some methods in it 
 */

// Taking area and perimeter of rectangle

struct Rect{
    width : i32,
    height: i32,
}

// Implementing the structs
impl Rect{
    // Some methods like functions
    fn area(&self) -> i32{
        self.width * self.height
    }
    // The self is nothing but this keyword this.width like that

    fn perimeter(&self) ->i32{
        2 * (self.width + self.height)
    }

    // and remeber dont add ; in fucntion that means the end statement 

}

fn main(){
    let rect =  Rect {
        width : 10,
        height : 20,
    };

    println!("Area is {}" ,rect.area());
    println!("Perimeter  is {}" ,rect.perimeter());

// Sry for the syntax issue

}