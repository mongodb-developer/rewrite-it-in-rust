use std::{cmp::max, convert::TryFrom, time::Duration};

use crate::model::{Cocktail, CocktailJSON};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
use futures::stream::StreamExt;
use mongodb;
use mongodb::Collection;
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};
use tokio;

fn default_page() -> i64 {
    1
}

#[derive(Serialize)]
struct ErrorJSON {
    error: String,
}

impl ErrorJSON {
    fn new(message: String) -> Self {
        Self { error: message }
    }
}

#[derive(Deserialize)]
struct ListCocktailsParams {
    #[serde(default = "default_page")]
    page: i64,
}

#[derive(Serialize)]
struct ListCocktailsResult {
    recipes: Vec<CocktailJSON>,
    links: Links,
}

impl ListCocktailsResult {
    fn new(
        recipes: Vec<Cocktail>,
        page: i64,
        page_size: i64,
        cocktail_count: i64,
        url_for: fn(i64) -> String,
    ) -> Self {
        Self {
            recipes: recipes.into_iter().map(CocktailJSON::from).collect(),
            links: Links::new(page, (cocktail_count / page_size) + 1, url_for),
        }
    }
}

#[derive(Serialize)]
struct Link {
    href: String,
}

impl Link {
    fn new(href: String) -> Self {
        Self { href: href }
    }
}

#[derive(Serialize)]
struct Links {
    #[serde(rename = "self")]
    this: Link,
    first: Link,
    last: Link,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<Link>,
}

impl Links {
    fn new(page: i64, max_page: i64, url_for: fn(i64) -> String) -> Self {
        return Links {
            this: Link::new(url_for(page)),
            first: Link::new(url_for(1)),
            last: Link::new(url_for(max_page)),
            prev: if page > 1 {
                Some(Link::new(url_for(page - 1)))
            } else {
                None
            },
            next: if page < max_page {
                Some(Link::new(url_for(page + 1)))
            } else {
                None
            },
        };
    }
}

#[get("/cocktails/")]
async fn list_cocktails(
    request: HttpRequest,
    recipes: web::Data<Collection<Cocktail>>,
    web::Query(params): web::Query<ListCocktailsParams>,
) -> impl Responder {
    const PAGE_SIZE: i64 = 10;

    let page = max(params.page, 1);

    let cocktail_count = match recipes.count_documents(None, None).await {
        Ok(cocktail_count) => cocktail_count,
        Err(e) => {
            println!("Error while listing cocktails: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let result = recipes
        .find(
            None,
            FindOptions::builder()
                .sort(doc! { "name": 1 })
                .skip((page - 1) * PAGE_SIZE)
                .limit(PAGE_SIZE)
                .build(),
        )
        .await;
    match result {
        Ok(cursor) => {
            let results: Vec<Result<_, _>> = cursor.collect().await;
            let result: Result<Vec<_>, _> = results.into_iter().collect();
            match result {
                Ok(cocktails) => {
                    return HttpResponse::Ok().json(ListCocktailsResult::new(
                        cocktails,
                        page,
                        PAGE_SIZE,
                        cocktail_count,
                        |i| format!("/cocktails/?page={}", i),
                    ))
                }
                Err(e) => {
                    println!("Error while listing cocktails: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
        Err(e) => {
            println!("Error while listing cocktails: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/cocktails/")]
async fn new_cocktail(
    recipes: web::Data<Collection<Cocktail>>,
    Json(cocktail_json): Json<CocktailJSON>,
) -> impl Responder {
    let mut cocktail = match Cocktail::try_from(cocktail_json) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error parsing cocktail: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    cocktail.date_added = Some(chrono::Utc::now());

    let insert_result = match recipes.insert_one(cocktail, None).await {
        Ok(insert_result) => insert_result,
        Err(e) => {
            println!("Error retrieving inserted cocktail: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match recipes
        .find_one(doc! {"_id": insert_result.inserted_id}, None)
        .await
    {
        Ok(result) => {
            if let Some(recipe) = result {
                HttpResponse::Ok().json(CocktailJSON::from(recipe))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            println!("Error retrieving saved cocktail {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/cocktails/{slug}")]
async fn get_cocktail(
    recipes: web::Data<Collection<Cocktail>>,
    web::Path(slug): web::Path<String>,
) -> impl Responder {
    match recipes.find_one(doc! { "slug": slug }, None).await {
        Ok(result) => {
            if let Some(recipe) = result {
                HttpResponse::Ok().json(CocktailJSON::from(recipe))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/cocktails/{slug}")]
async fn update_cocktail(
    recipes: web::Data<Collection<Cocktail>>,
    web::Path(slug): web::Path<String>,
    Json(cocktail_json): Json<CocktailJSON>,
) -> impl Responder {
    let mut cocktail = match Cocktail::try_from(cocktail_json) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error parsing cocktail: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    cocktail.date_updated = Some(chrono::Utc::now());

    println!("Cocktail prepared");
    let update_result = match recipes
        .update_one(
            doc! {"slug": &slug},
            doc! {"$set": mongodb::bson::to_bson(&cocktail).unwrap()},
            None,
        )
        .await
    {
        Ok(update_result) => update_result,
        Err(e) => {
            println!("Error updating cocktail: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    println!("Matched cocktails: {}", &update_result.matched_count);

    if update_result.matched_count == 1 {
        tokio::time::delay_for(Duration::from_secs(1)).await;
        match recipes.find_one(doc! { "slug": &slug }, None).await {
            Ok(result) => {
                if let Some(recipe) = result {
                    HttpResponse::Ok().json(CocktailJSON::from(recipe))
                } else {
                    println!("Cannot retrieve updated document: {}", &slug);
                    HttpResponse::InternalServerError().finish()
                }
            }
            Err(e) => {
                println!("Error while getting, {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/cocktails/{slug}")]
async fn delete_cocktail(
    recipes: web::Data<Collection<Cocktail>>,
    web::Path(slug): web::Path<String>,
) -> impl Responder {
    match recipes
        .find_one_and_delete(doc! { "slug": slug }, None)
        .await
    {
        Ok(result) => {
            if let Some(recipe) = result {
                HttpResponse::Ok().json(CocktailJSON::from(recipe))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
