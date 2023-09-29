use postgres::{Client, NoTls};

pub fn save_user(user: &User) -> Result<(), String> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)
        .map_err(|err| format!("Failed to connect to PostgreSQL: {}", err))?;

    client
        .execute(
            "INSERT INTO users (id, name, age) VALUES ($1, $2, $3)",
            &[&user.id, &user.name, &user.age],
        )
        .map_err(|err| format!("Failed to execute query: {}", err))?;

    Ok(())
}

pub fn save_pizza_order(pizza_order: &PizzaOrder) -> Result<(), String> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)
        .map_err(|err| format!("Failed to connect to PostgreSQL: {}", err))?;

    client
        .execute(
            "INSERT INTO pizza_orders (id, size, quantity) VALUES ($1, $2, $3)",
            &[&pizza_order.id, &pizza_order.size, &pizza_order.quantity],
        )
        .map_err(|err| format!("Failed to execute query: {}", err))?;

    Ok(())
}

