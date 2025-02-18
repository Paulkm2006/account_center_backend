use mongodb::{bson::{doc, oid}, Database};
use serde_json::{Value, json};
use totp_rs::{Secret, TOTP};

use crate::dto::*;



pub async fn create_auth(db: Database, a_type: String, data: Value) -> Result<oid::ObjectId, Box<dyn std::error::Error>> {
	
	let collection = db.collection::<AuthInfo>("auths");
	let auth = match a_type.as_str() {
		"totp" => {
			let totp = TOTP::new_unchecked(
				match data["algorithm"].as_str().unwrap() {
					"SHA1" => totp_rs::Algorithm::SHA1,
					"SHA256" => totp_rs::Algorithm::SHA256,
					"SHA512" => totp_rs::Algorithm::SHA512,
					_ => return Err("Invalid algorithm".into())}, 
				data["digits"].as_u64().unwrap() as usize,
				1, 30, 
				Secret::Encoded(data["secret"].as_str().unwrap().to_string()).to_bytes().unwrap());
			AuthInfo {
				id: oid::ObjectId::new(),
				auth_type: AuthType::TOTP(totp),
			}
		},
		"email" => {
			let email = AuthOwnerInfo {
				name: data["name"].as_str().unwrap().to_string(),
				email: Some(data["email"].as_str().unwrap().to_string()),
				phone: None,
				qq: data["qq"].as_str().unwrap().to_string(),
				comment: match data["comment"].as_str() {
					Some(comment) => Some(comment.to_string()),
					None => None,
				},
			};
			AuthInfo {
				id: oid::ObjectId::new(),
				auth_type: AuthType::Email(email),
			}
		},
		"phone" => {
			let phone = AuthOwnerInfo {
				name: data["name"].as_str().unwrap().to_string(),
				email: None,
				phone: Some(data["phone"].as_str().unwrap().to_string()),
				qq: data["qq"].as_str().unwrap().to_string(),
				comment: match data["comment"].as_str() {
					Some(comment) => Some(comment.to_string()),
					None => None,
				},
			};
			AuthInfo {
				id: oid::ObjectId::new(),
				auth_type: AuthType::Phone(phone),
			}
		},
		_ => return Err("Invalid auth type".into()),
	};

	let result = collection.insert_one(auth).await?;
	Ok(result.inserted_id.as_object_id().unwrap().clone())
}

pub async fn get_auth(db: Database, id: oid::ObjectId) -> Result<Option<Value>, Box<dyn std::error::Error>> {
	let collection = db.collection::<AuthInfo>("auths");
	let auth = collection.find_one(doc!{"_id": id}).await?;
	let auth = match auth {
		Some(auth) => auth,
		None => return Ok(None),
	};

	match auth.auth_type {
		AuthType::TOTP(totp) => {
			let code = totp.generate_current()?;
			Ok(Some(json!({"type": "totp", "data": code})))
		},
		AuthType::Email(email) => {
			Ok(Some(json!({"type": "email", "data": email})))
		},
		AuthType::Phone(phone) => {
			Ok(Some(json!({"type": "phone", "data": phone})))
		},
	}
}

pub async fn delete_auth(db: Database, id: oid::ObjectId) -> Result<(), Box<dyn std::error::Error>> {
	let collection = db.collection::<AuthInfo>("auths");
	collection.delete_one(doc!{"_id": id}).await?;
	Ok(())
}
