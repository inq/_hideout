use bson::doc;
use serde::{Deserialize, Serialize};

pub struct Workouts {
    pub(super) db: mongodb::Database,
}

impl Workouts {
    pub async fn all(self) -> Result<Vec<Workout>, mongodb::error::Error> {
        use futures::StreamExt;

        let cursor = self.db.collection("workouts").find(None, None).await?;
        let res: Vec<Result<_, _>> = cursor.collect().await;
        res.into_iter()
            .map(|doc| -> Result<Workout, _> {
                bson::from_bson(bson::Bson::Document(doc?))
                    .map_err(|e| mongodb::error::ErrorKind::BsonDecode(e).into())
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workout {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    name: String,
    description: String,
    #[serde(rename = "withBarbell")]
    with_barbell: bool,
}

impl Workout {
    pub fn new(
        id: Option<bson::oid::ObjectId>,
        name: String,
        description: String,
        with_barbell: bool,
    ) -> Self {
        Self {
            id,
            name,
            description,
            with_barbell,
        }
    }
}
