use chrono::{DateTime, Utc};

use super::schema::profiles;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "profiles"]
pub struct Profile {
  pub id: i32,
  pub user_id: i32,
  pub name: String,
  pub bio: String,
  pub avatar_url: String,
  pub public_repos: i32,
  pub public_gists: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
