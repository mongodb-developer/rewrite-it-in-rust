use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Quantity {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ingredient {
    name: String,
    quantity: Option<Quantity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cocktail {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    slug: String,
    name: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    slug: String,
    name: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_added: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_updated: Option<DateTime<Utc>>,
}

impl From<Cocktail> for CocktailJSON {
    fn from(cocktail: Cocktail) -> Self {
        Self {
            id: cocktail.id.map(|id| id.to_hex()),
            slug: cocktail.slug,
            name: cocktail.name,
            ingredients: cocktail.ingredients,
            instructions: cocktail.instructions,
            date_added: cocktail.date_added,
            date_updated: cocktail.date_updated,
        }
    }
}

impl TryFrom<CocktailJSON> for Cocktail {
    type Error = mongodb::bson::oid::Error;

    fn try_from(cocktail: CocktailJSON) -> Result<Self, Self::Error> {
        let cocktail_oid = match cocktail.id {
            Some(id) => Some(ObjectId::with_string(&id)?),
            None => None,
        };

        Ok(Self {
            id: cocktail_oid,
            slug: cocktail.slug,
            name: cocktail.name,
            ingredients: cocktail.ingredients,
            instructions: cocktail.instructions,
            date_added: cocktail.date_added,
            date_updated: cocktail.date_updated,
        })
    }
}
