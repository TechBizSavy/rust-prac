// Enums in Rust
/*
    Making an enum and assigning some value 
    lets make one enum and mostly enum are used we we want to make
    same types of enumrate 
    example 

    Direction includes north , west ,south like that

*/

enum Shape {
    Rectangle(f64 , f64) ,
    Circle(f64) ,
}

// We are making a function that takes area of this enums 

fn main(){

    let rec = Shape::Rectangle(4.2, 1.2);
    let rec_area = calculator(rec);
    println!("Area of rec {}" , rec_area);

    let cir = Shape::Circle(4.0);
    let cir_area = calculator(cir);
    println!("Area of cir {}" , cir_area);

}


//F represent float 
// shape is an argument which contains then Shape circle and rectangle

fn calculator(shape: Shape) -> f64{
    match shape {
        Shape::Rectangle(a ,b )=> a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}


