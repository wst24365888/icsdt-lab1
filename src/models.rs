use std::sync::Arc;

use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

use crate::schema::lab1;

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, Queryable)]
#[diesel(table_name = lab1)]
pub struct Lab1Data {
    pub id: i32,
    pub name: String,
}

pub struct AppState {
    pub db_pool: Arc<crate::DbPool>,
}