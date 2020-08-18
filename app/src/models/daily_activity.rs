use bson::doc;
use serde::{Deserialize, Serialize};

pub struct DailyActivities {
    pub(super) db: mongodb::Database,
}

impl DailyActivities {
    pub async fn all(self) -> Result<Vec<DailyActivity>, mongodb::error::Error> {
        use futures::StreamExt;

        let cursor = self
            .db
            .collection("dailyActivities")
            .find(None, None)
            .await?;
        let res: Vec<Result<_, _>> = cursor.collect().await;
        res.into_iter()
            .map(|doc| -> Result<DailyActivity, _> {
                bson::from_bson(bson::Bson::Document(doc?))
                    .map_err(|e| mongodb::error::ErrorKind::BsonDecode(e).into())
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyActivity {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    date: bson::DateTime,

    workouts: Vec<DailyWorkout>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyWorkout {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    workout_id: bson::oid::ObjectId,
}

impl DailyWorkout {
    pub fn new(id: bson::oid::ObjectId, workout_id: bson::oid::ObjectId) -> Self {
        Self { id, workout_id }
    }
}
