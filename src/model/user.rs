use mongodb::{bson::{self, doc}, Database};
use crate::dto::UserInfo;
use std::time::SystemTime;


pub async fn check_user(
	user: &UserInfo,
	db: Database
) -> Result<Option<String>, mongodb::error::Error> {
	let collection = db.collection::<UserInfo>("users");

	let filter = doc! {
		"sub": user.sub.clone()
	};

	let result = collection.find_one(filter).await?;

	match result {
		Some(i) => {
			collection.update_one(
				doc! {
					"sub": user.sub.clone()
				},
				doc! {
					"$set": {
						"last_login": bson::DateTime::from(SystemTime::now())
					}
				},
			).await?;
			Ok(Some(i.last_login.unwrap().to_string()))
		},
		None => {
			create_user(user.clone(), db).await?;
			Ok(None)
		}
	}
}

pub async fn get_user_info(
	user: String,
	db: Database
) -> Result<Option<UserInfo>, mongodb::error::Error> {
	let collection = db.collection::<UserInfo>("users");

	let filter = doc! {
		"sub": user
	};

	let result = collection.find_one(filter).await?;

	Ok(result)
}

pub async fn create_user(
	user: UserInfo,
	db: Database
) -> Result<(), mongodb::error::Error> {
	let collection = db.collection::<UserInfo>("users");

	let mut user_db = user.clone();
	user_db.iat = None;
	user_db.exp = None;

	collection.insert_one(user_db).await?;

	Ok(())
}