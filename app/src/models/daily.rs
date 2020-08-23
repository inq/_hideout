use bson::doc;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    created_at: bson::DateTime,
    workout_id: bson::oid::ObjectId,
}

impl Activity {
    pub fn new(
        id: Option<bson::oid::ObjectId>,
        
    ) -> Self {
        Self {
            id,
            name,
            description,
            with_barbell,
        }
    }
}
