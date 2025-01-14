// Define a struct for an item in the shopping cart
struct Item {
    name: String,
    price: f64,
}

// Define a struct for the shopping cart
struct Cart {
    items: Vec<Item>,  // A vector to hold multiple items
}

impl Cart {
    // Create a new empty cart
    fn new() -> Self {
        Cart { items: Vec::new() }
    }

    // Add an item to the cart
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // Calculate the total price of items in the cart
    fn total_price(&self) -> f64 {
        let mut total = 0.0;
        for item in &self.items {
            total += item.price;
        }
        total
    }

    // Display the items in the cart
    fn display_items(&self) {
        println!("Items in the cart:");
        for item in &self.items {
            println!("{} - ${:.2}", item.name, item.price);
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
