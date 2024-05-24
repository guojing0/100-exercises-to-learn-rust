// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: isize,
    unit_price: isize,
}

impl Order {
    pub fn new(product_name: String, quantity: isize, unit_price: isize) -> Order {
        Self::check_name(&product_name);
        Self::check_quantity(&quantity);
        Self::check_unit_price(&unit_price);

        Order {
            product_name, quantity, unit_price
        }
    }

    pub fn total(&self) -> isize {
        &self.quantity * &self.unit_price
    }

    fn check_name(name: &String) {
        if name.is_empty() {
            panic!("Name cannot be empty!");
        }
        if name.len() > 300 {
            panic!("Name too long!");
        }
    }

    fn check_quantity(quantity: &isize) {
        if *quantity <= 0 {
            panic!();
        }
    }

    fn check_unit_price(unit_price: &isize) {
        if *unit_price <= 0 {
            panic!();
        }
    }

    pub fn set_product_name(&mut self, name: String) {
        Self::check_name(&name);

        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: isize) {
        Self::check_quantity(&quantity);

        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: isize) {
        Self::check_unit_price(&unit_price);

        self.unit_price = unit_price;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &isize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &isize {
        &self.unit_price
    }
}
