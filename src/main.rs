// use std::path::Path;
use actix_web::web::{Data, Path};
use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use error::PizzaError;
use models::Pizza;
use surrealdb::method::Update;
// use models::Buy_Pizza_Request;
mod models;
mod db;
use crate::db::Database;
use crate::models::pizza::{Buy_Pizza_Request, UpdatePizzaURL};
use validator::Validate;
use uuid;
mod error;




#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>,PizzaError> {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        // Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        // None => HttpResponse::Ok().body("Error")
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzaFound),
        }
}

#[post("/buy_pizza")]
async fn buy_pizza(body: Json<Buy_Pizza_Request>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_)=> {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = db.add_pizza(Pizza::new(
                String::from(new_uuid),
                pizza_name
            )).await;

            // DO the match to check if operation was successful in creating pizza
            match new_pizza {
                Some(created) => {
                    // HttpResponse::Ok().body(format!("Created new pizza: {:?}", created))
                    Ok(Json(created))
                },
                None => 
                    // HttpResponse::Ok().body("Error creating pizza"),
                    Err(PizzaError::PizzaCreationError)
            

            }            
        },
        Err(_) => Err(PizzaError::PizzaCreationError),
    }
    // HttpResponse::Ok().body("Pizza on its way")
}

#[patch("/update_piza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>,
db:Data<Database>
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = db.update_piza( uuid).await;
    // HttpResponse::Ok().body(format!("Pizza updated with uuid: {uuid}"))
    match update_result {
        Some(Update_pizza) => Ok(Json(Update_pizza)),
        None => Err(PizzaError::NoSuchPizzaFound),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}