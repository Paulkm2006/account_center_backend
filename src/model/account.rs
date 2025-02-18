use mongodb::{bson::{self, doc, oid}, Database};
use futures::stream::TryStreamExt;
use serde_json::{json, Value};

use crate::dto::{*};

pub async fn get_all_accounts(db: Database) -> Result<Vec<Value>, mongodb::error::Error> {
	let collection = db.collection::<AccountInfo>("accounts");
	let mut cursor = collection.find(doc!{}).await?;
	let mut accounts = Vec::new();
	while let Some(account) = cursor.try_next().await? {
		accounts.push(json!({"name": account.name, "id": account.id.unwrap().to_hex()}));
	}
	Ok(accounts)
}

pub async fn get_account(db: Database, id: oid::ObjectId) -> Result<Option<AccountInfo>, mongodb::error::Error> {
	let collection = db.collection::<AccountInfo>("accounts");
	let account = collection.find_one(doc!{"_id": id}).await?;
	match account {
		Some(account) => Ok(Some(account)),
		None => Ok(None),
	}
}

pub async fn create_account(db: Database, account: AccountInfo) -> Result<oid::ObjectId, mongodb::error::Error> {
	let collection = db.collection::<AccountInfo>("accounts");
	let result = collection.insert_one(account).await?;
	Ok(result.inserted_id.as_object_id().unwrap().clone())
}

pub async fn update_account(db: Database, id: oid::ObjectId, account: AccountInfo) -> Result<(), mongodb::error::Error> {
	let collection = db.collection::<AccountInfo>("accounts");
	let account_doc = bson::to_document(&account)?;
	collection.update_one(doc!{"_id": id}, doc!{"$set": account_doc}).await?;
	Ok(())
}

pub async fn delete_account(db: Database, id: oid::ObjectId) -> Result<(), mongodb::error::Error> {
	let collection = db.collection::<AccountInfo>("accounts");
	collection.delete_one(doc!{"_id": id}).await?;
	Ok(())
}