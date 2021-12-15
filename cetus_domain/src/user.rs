use chrono::{DateTime, Utc};

use super::schema::users;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub gh_username: String,
  pub gh_token: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
