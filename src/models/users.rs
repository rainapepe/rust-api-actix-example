use crate::services::mongo;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_bson, oid, Bson, Document},
    error::Error,
    results::InsertOneResult,
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32,
}

async fn get_collection() -> Collection {
    mongo::get_collection("users").await
}

pub async fn insert_one(name: &String, age: u32) -> Result<InsertOneResult, Error> {
    let collection = get_collection().await;

    let user = doc! {
        "name": name,
        "age": age
    };

    let result = collection.insert_one(user, None).await;

    result
}

pub async fn get_one(id: &String) -> Result<Option<User>, Error> {
    let collection = get_collection().await;

    let result = collection
        .find_one(
            doc! { "_id":  oid::ObjectId::with_string(id).unwrap() },
            None,
        )
        .await;

    match result {
        Ok(Some(document)) => {
            let user: User = from_bson(Bson::Document(document)).unwrap();

            Ok(Some(user))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
    // result
}

pub async fn get_all() -> Vec<Document> {
    let collection = get_collection().await;
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results = Vec::<Document>::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {}
        }
    }

    results
}
