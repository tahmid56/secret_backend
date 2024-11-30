use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::types::Json;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "encryption_method")]
pub enum EncryptionMethod {
    AES256,
    Chacha20,
    Blowfish,
    DESTripleDES,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
pub struct DbConnection {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub encryption_method: Option<EncryptionMethod>,
    pub keys: Option<Vec<u8>>,
    pub api_key: Option<String>,
    pub db_connection: Option<Json<DbConnection>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Secret{
    pub id: uuid::Uuid,
    pub secret_name: String,
    pub encryption_secret_value: Vec<u8>,
    pub version: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SecretVersion{
    pub id: uuid::Uuid,
    pub secret_id: uuid::Uuid,
    pub secret_name: String,
    pub encryption_secret_value: Vec<u8>,
    pub version: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}