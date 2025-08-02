use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Females,
    Trans,
    Notdefined,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
    Voyager,
    Warden,
    Oracle,
    Strider,
    Architect,
}
// Implement Display for both
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_deserializing)]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub age: i8,
    pub gender: Gender,

    #[serde(default, skip_deserializing)]
    pub joined_at: Option<DateTime<Utc>>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}
