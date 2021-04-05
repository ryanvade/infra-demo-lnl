use actix_web::dev::Body;
use actix_web::web;
use bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub descr: String,
    pub created_at: DateTime,
    pub completed: bool,
}

impl std::convert::Into<String> for ToDo {
    fn into(self) -> String {
        return String::new();
    }
}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self);
        if let Ok(json) = json {
            return write!(f, "{}", json);
        }
        let err = json.err().unwrap();
        write!(f, "{:#?}", err)
    }
}

impl std::convert::Into<actix_web::dev::Body> for ToDo {
    fn into(self) -> Body {
        return Body::Bytes(web::Bytes::from(self.to_string()));
    }
}

impl ToDo {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "id": self.id.to_string(),
            "descr": self.descr,
            "createdAt": self.created_at.to_rfc3339(),
            "completed": self.completed
        })
    }
}
