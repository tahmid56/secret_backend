use validator::Validate;
use serde::{Serialize, Deserialize};
use crate::models::{EncryptionMethod, User};
use validator::{Validate, ValidationError};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto{
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(max = 6, message = "Password must be less than 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, Default)]
pub struct LoginUserDto{
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(max = 6, message = "Password must be less than 6 characters")
    )]
    pub password: String,
}

#[derive( Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub encryption_method: Option<EncryptionMethod>,
    pub api_keys: Option<String>,
    #[serde(rename = "dbConnectionExists")]
    pub db_connection_exists: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.username.clone(),
            email: user.email.clone(),
            encryption_method: user.encryption_method,
            api_keys: user.api_keys.clone(),
            db_connection_exists: user.db_connection.is_some(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub data: FilterUserDto,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response{
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct UserPasswordUpdateDto{
    #[validate(
        length(min = 1, message = "Password is required"),
        length(max = 6, message = "Password must be less than 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub confirm_password: String,
    #[validate(
        length(min = 1, message = "Old Password is required"),
        length(max = 6, message = "Password must be less than 6 characters")
    )]
    pub old_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DatabaseDto {
    #[validate(length(min = 1, message = "Host is required"))]
    pub host: String,
    #[validate(length(min = 1, max = 65535 message = "Port must be between 1 and 65535"))]
    pub port: u16,
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    #[validate(length(min = 1, message = "Database is required"))]
    pub database: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct EncryptionMethodDto {
    #[validate(custom = "validate_encryption_method")]
    pub encryption_method: EncryptionMethod,
}

fn validate_encryption_method(encryption_method: &EncryptionMethod) -> Result<(), ValidationError> {
    match encryption_method{
        EncryptionMethod::AES256
        | EncryptionMethod::Chacha20
        | EncryptionMethod::Blowfish
        | EncryptionMethod::DESTripleDES => Ok(()),
        _ => Err(ValidationError::new("Invalid encryption method")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SaveSecretDto {
    #[validate(length(min = 1, message = "Secret Name is required"))]
    pub secret_name: String,
    #[validate(length(min = 1, message = "Secret Value is required"))]
    pub secret_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct EditSecretDto {
    #[validate(length(min = 1, message = "Secret Name is required"))]
    pub secret_name: String,
    #[validate(length(min = 1, message = "Secret Value is required"))]
    pub secret_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct FilterSecretDto {
    #[validate(length(min = 1, message = "Secret Name is required"))]
    pub secret_name: String,
    #[validate(length(min = 1, message = "Secret Value is required"))]
    pub secret_value: String,
    pub id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RequestQuerySecretVersionDto {
    #[validate(range(min=1))]
    pub page: Option<usize>,
    #[validate(range(min=1, max = 50))]
    pub limit: Option<usize>,
    pub id: uuid::Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretResponse {
    pub id: String,
    pub secret_name: String,
    pub secret_value: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterSecretDto {
    pub id: String,
    pub secret_name: String,
    pub secret_value: String,
    pub version: i32,
}

impl FilterSecretDto {
    pub fn filter_secret(secret: &SecretResponse) -> Self {
        FilterSecretDto {
            id: secret.id.to_string(),
            secret_name: secret.secret_name.clone(),
            secret_value: secret.secret_value.clone(),
        }
    }

    pub fn filter_secrets(secret: &[SecretResponse]) -> Vec<FilterSecretDto> {
        secret
            .iter()
            .map(|secret| FilterSecretDto::filter_secret(secret))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResponseDto{
    pub secret: Vec<FilterSecretDto>,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuerySecretByKeyDto{
    pub key: String,
    pub secret: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuerySecretByKeyResponseDto{
    pub value: String
}