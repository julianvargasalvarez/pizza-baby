use crate::domain::user::User;

pub fn create_user(name: String, age: i32) -> Result<User, String> {
    if age <= 18 {
        return Err("Age must be greater than 18".to_string());
    }

    let user = User {
        id: 1, // You can generate the ID in your actual implementation
        name,
        age,
    };

    Ok(user)
}
