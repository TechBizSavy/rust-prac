struct  Item{
    name : String,
    price : f64,
}

struct  Cart{
    items : Vec<Item>,
}

impl  Cart {
    // Creatin a new empty cart
    fn new()-> Self {
        Cart { items: Vec::new() }
    }

    fn add_items(&mut self , item:Item){
        self.items.push(item);
    }

    fn total_price(&self)->f64{
        let mut total = 0.0;
        for  item  in &self.items  {
            total += item.price;
        }
        total
    }

    fn display_items(&self){
        println!("Items in Cart " );
        for item in &self.items  {
            println!("{} - ${:.2} " ,item.name, item.price );
        }
    }


}


fn main(){
    let mut cart = Cart::new();
    cart.add_items(Item{
        name : String::from("Apple"),
        price : 1.20,
    });
    
    cart.add_items(Item{
        name : String::from("Milk"),
        price : 0.20,
    });

    cart.display_items();
    let total = cart.total_price();
    println!("Total price &{:.2} " , total);
}