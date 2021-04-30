mod model;
mod routes;

use actix_web::{middleware, App, HttpServer};
use mongodb;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse(
        &env::var("MONGO_URI").expect("The MONGO_URI environment variable must be set."),
    )
    .await
    .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let recipes: Collection<model::Cocktail> =
        client.database("cocktails").collection_with_type("recipes");

    HttpServer::new(move || {
        App::new()
            .data(recipes.clone())
            .wrap(middleware::Logger::default())
            .service(routes::list_cocktails)
            .service(routes::get_cocktail)
            .service(routes::new_cocktail)
            .service(routes::update_cocktail)
            .service(routes::delete_cocktail)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
