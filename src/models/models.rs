use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Penguin {
    pub id: uuid::Uuid,
    pub penguin_name: String,
    pub age: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreatePenguin {
    pub penguin_name: String,
    pub age: i16,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UpdatePenguin {
    pub penguin_name: String,
    pub age: i16,
}



