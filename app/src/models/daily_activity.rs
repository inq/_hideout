use bson::{doc, to_bson, Bson};
use serde::{Deserialize, Serialize};

pub struct DailyActivities {
    pub(super) db: mongodb::Database,
}

const COLLECTION_NAME: &str = "dailyActivities";

impl DailyActivities {
    pub async fn all(self) -> Result<Vec<DailyActivity>, mongodb::error::Error> {
        use futures::StreamExt;

        let cursor = self.db.collection(COLLECTION_NAME).find(None, None).await?;
        let res: Vec<Result<_, _>> = cursor.collect().await;
        res.into_iter()
            .map(|doc| -> Result<DailyActivity, _> {
                bson::from_bson(bson::Bson::Document(doc?))
                    .map_err(|e| mongodb::error::ErrorKind::BsonDecode(e).into())
            })
            .collect()
    }

    pub async fn create(
        self,
        date: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), mongodb::error::Error> {
        println!("{:?}", date);

        if let Bson::Document(document) = to_bson(&DailyActivity {
            id: None,
            date: date.into(),
            workouts: vec![],
        })
        .unwrap()
        {
            let _res = self
                .db
                .collection(COLLECTION_NAME)
                .insert_one(document, None)
                .await?;
            Ok(())
        } else {
            unreachable!();
        }
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
