

use mongodb::bson::{oid, DateTime};
use serde::{Deserialize, Serialize};
use totp_rs::TOTP;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
	pub nickname: String,
	pub sub: String,
	pub rol: String,
	pub email: String,
	pub phone: String,
	pub picture: String,
	pub group: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_login: Option<DateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exp: Option<i64>,  // JWT expiration time
	#[serde(skip_serializing_if = "Option::is_none")]
	pub iat: Option<i64>,  // JWT issued at time
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccountInfo {
	#[serde(rename = "_id")]
	pub id: Option<oid::ObjectId>,
	pub name: String,
	pub account: String,
	pub password: String,
	pub created_by: Option<String>,
	pub created_at: Option<DateTime>,
	pub updated_by: Option<String>,
	pub updated_at: Option<DateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comment: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auth_id: Option<oid::ObjectId>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthOwnerInfo {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,
	pub qq: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comment: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AuthType {
	TOTP(TOTP),
	Email(AuthOwnerInfo),
	Phone(AuthOwnerInfo),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthInfo {
	#[serde(rename = "_id")]
	pub id: oid::ObjectId,
	pub auth_type: AuthType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthResponse<T> {
	#[serde(rename = "type")]
	pub a_type: String,
	pub data: T,
}


#[derive(Deserialize)]
pub struct QueryParams {
	pub id: String,
}