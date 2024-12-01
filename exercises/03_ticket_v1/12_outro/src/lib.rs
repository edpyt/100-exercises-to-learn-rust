// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
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

type ProductName = String;
type Quantity = u32;
type UnitPrice = u32;

pub struct Order {
    product_name: ProductName,
    quantity: Quantity,
    unit_price: UnitPrice,
}

impl Order {
    pub fn new(product_name: ProductName, quantity: Quantity, unit_price: UnitPrice) -> Self {
        let (product_name, quantity, unit_price) =
            Self::validate_fields(product_name, quantity, unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}

impl Order {
    pub fn product_name(&self) -> &ProductName {
        &self.product_name
    }

    pub fn set_product_name(&mut self, product_name: ProductName) {
        self.product_name = Self::validate_product_name(product_name);
    }

    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }

    pub fn set_quantity(&mut self, quantity: Quantity) {
        self.quantity = Self::validate_quantity(quantity);
    }

    pub fn unit_price(&self) -> &UnitPrice {
        &self.unit_price
    }

    pub fn set_unit_price(&mut self, unit_price: UnitPrice) {
        self.unit_price = Self::validate_unit_price(unit_price);
    }
}

impl Order {
    fn validate_fields(
        product_name: ProductName,
        quantity: Quantity,
        unit_price: UnitPrice,
    ) -> (ProductName, Quantity, UnitPrice) {
        (
            Self::validate_product_name(product_name),
            Self::validate_quantity(quantity),
            Self::validate_unit_price(unit_price),
        )
    }

    fn validate_product_name(product_name: ProductName) -> ProductName {
        if product_name.is_empty() {
            panic!("product_name can't be empty!")
        } else if product_name.len() > 300 {
            panic!("product_name bytes can't be longer than 300!")
        }
        product_name
    }

    fn validate_quantity(quantity: Quantity) -> Quantity {
        if quantity <= 0 {
            panic!("quantity can't be lower than zero!")
        }
        quantity
    }

    fn validate_unit_price(unit_price: UnitPrice) -> UnitPrice {
        if unit_price <= 0 {
            panic!("unit_price can't be lower than zero!")
        }
        unit_price
    }
}
