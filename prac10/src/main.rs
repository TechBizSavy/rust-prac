// Shopping Cart Program in rust
/*
We will make a structs named 
Item , Cart and will impl Cart 
add , new , total price , display price ;


*/

struct Item{
    name : String,
    price : f64,
}

struct Cart{
    items : Vec<Item>,
    // This will make a dynamic list of item 
}



impl Cart {
    // Creating a new func
    // This fucn will take it owns value 
    fn new()-> Self{
        Cart { items: Vec::new() }
    }

// Adding item 
    fn add_item(&mut self , item : Item){
        self.items.push(item);
    }

    // Creating a func for totaling of price 
// This func add the item and make the index to zero after adding 
    fn total_price(&self)-> f64{
        let mut total = 0.0;
        for item in &self.items  {
         total += item.price;
        }
        total     
    }

    // Creating a display func
    fn display_items(&self){
        println!("Items in the cart aree !");
        for item in &self.items  {
            println!("{} - ${:.2} ", item.name , item.price);
        }
    }


}


fn main(){
    let mut cart = Cart::new();

    cart.add_item(Item{
        name : String::from("Orange"),
        price: 2.2,
    });

    cart.add_item(Item{
        name : String::from("Chocolates"),
        price: 5.52,
    });

    cart.add_item(Item{
        name : String::from("Bread"),
        price: 0.5,
    });

    cart.display_items();
    let total =  cart.total_price();
    // {:.2 } is nothing but two decimal more
    println!("Total price is {:.2} " , total)
}