use crate::domain::pizza_order::PizzaOrder;

pub fn order_pizza(size: String, quantity: i32) -> Result<PizzaOrder, String> {
    if quantity <= 0 {
        return Err("Quantity must be greater than zero".to_string());
    }

    let pizza_order = PizzaOrder {
        id: 1, // You can generate the ID in your actual implementation
        size,
        quantity,
    };

    Ok(pizza_order)
}

