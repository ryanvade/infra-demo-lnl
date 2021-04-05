use bson::{doc, oid::ObjectId, Document};
use futures::stream::StreamExt;
use mongodb::options::FindOptions;
use mongodb::results::InsertOneResult;
use mongodb::{error::Result, Collection};
use std::str::FromStr;

use super::todo::ToDo;

#[derive(Clone)]
pub struct ToDoService {
    collection: Collection,
}

impl ToDoService {
    pub fn new(collection: Collection) -> ToDoService {
        ToDoService { collection }
    }

    pub async fn create(&self, todo: Document) -> Result<InsertOneResult> {
        self.collection.insert_one(todo, None).await
    }

    pub async fn get(&self, id: &String) -> Result<Option<ToDo>> {
        let id = ObjectId::from_str(id);
        if id.is_err() {
            return Ok(None);
        }
        let filter = doc! {
            "_id": id.unwrap()
        };
        let result = self.collection.find_one(filter, None).await;
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let result = result.unwrap();
        match result {
            None => return Ok(None),
            Some(doc) => {
                let todo = bson::from_document::<ToDo>(doc).unwrap();
                return Ok(Some(todo));
            }
        }
    }

    pub async fn delete(&self, id: &String) -> Result<Option<i64>> {
        let id = ObjectId::from_str(id);
        if id.is_err() {
            return Ok(None);
        }
        let filter = doc! {
            "_id": id.unwrap()
        };
        let result = self.collection.delete_one(filter, None).await;
        match result {
            Ok(result) => {
                if result.deleted_count < 1 {
                    return Ok(None);
                }
                return Ok(Some(result.deleted_count));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn list(&self, last_id: Option<&String>) -> Result<Option<Vec<ToDo>>> {
        let mut filter = doc! {};
        if let Some(last_id) = last_id {
            let last_id = ObjectId::from_str(last_id);
            if last_id.is_err() {
                return Ok(None);
            }
            let last_id = last_id.unwrap();
            filter = doc! {
                "_id": {
                    "$gt": last_id
                }
            };
        }
        let options = FindOptions::builder().limit(1).build();
        let result = self.collection.find(filter, options).await;
        match result {
            Ok(result) => {
                let results: Vec<ToDo> = result
                    .filter_map(|r| async move {
                        if r.is_ok() {
                            return Some(r);
                        }
                        return None;
                    })
                    .map(|r| r.unwrap())
                    .map(|d| bson::from_document(d).unwrap())
                    .collect()
                    .await;
                return Ok(Some(results));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
