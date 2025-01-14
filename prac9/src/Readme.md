struct Item{
    name : String,
    price : f64,
}


struct  Cart{
    items : Vec<Item>,
}


impl Cart {
// Creating a new empty cart 
    fn new() -> Self{
        Cart { items: Vec::new() }
    }


    fn add_item(&mut self , item : Item){
        self.items.push(item);
    }

    fn total_price(&self)->f64{
        let mut total = 0.0;
        for item in &self.items{
            total += item.price;
        }
        total
    }


    fn display_items(&self){
        println!("Items in cart ");
        for item in &self.items{
            println!("{} - ${:.2}", item.name , item.price);
        }
    }
}





fn main() {
      // Create a new shopping cart
      let mut cart = Cart::new();

      // Add items to the cart
      cart.add_item(Item {
          name: String::from("Apple"),
          price: 0.50,
      });
      cart.add_item(Item {
          name: String::from("Bread"),
          price: 1.20,
      });
      cart.add_item(Item {
          name: String::from("Milk"),
          price: 2.00,
      });
  
      // Display items and total price
      cart.display_items();
      let total = cart.total_price();
      println!("Total price: ${:.2}", total);

}