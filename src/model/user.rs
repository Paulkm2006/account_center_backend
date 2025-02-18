use actix_web::web;
use mongodb::{bson::doc, Client};
use crate::dto::UserInfo;
use std::time::SystemTime;


pub async fn check_user(
	user: &UserInfo,
	mongo_client: web::Data<Client>,
) -> Result<Option<String>, mongodb::error::Error> {
	let db = mongo_client.database("account_center");
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
						"last_login": SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
					}
				},
			).await?;
			Ok(Some(i.last_login.unwrap()))
		},
		None => Ok(None),
	}
}