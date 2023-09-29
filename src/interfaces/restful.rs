use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use pizza_baby::use_cases::{create_user, order_pizza};

async fn create_user_handler(info: web::Json<CreateUserRequest>) -> impl Responder {
    let result = create_user::create_user(info.name.clone(), info.age);
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

async fn order_pizza_handler(info: web::Json<OrderPizzaRequest>) -> impl Responder {
    let result = order_pizza::order_pizza(info.size.clone(), info.quantity);
    match result {
        Ok(pizza_order) => HttpResponse::Ok().json(pizza_order),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

#[derive(serde::Deserialize)]
struct CreateUserRequest {
    name: String,
    age: i32,
}

#[derive(serde::Deserialize)]
struct OrderPizzaRequest {
    size: String,
    quantity: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user", web::post().to(create_user_handler))
            .route("/pizza-order", web::post().to(order_pizza_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
