use actix_web::{web, HttpResponse, Responder};

use mongodb::bson::{oid::ObjectId, DateTime};
use mongodb::Client;
use serde_json::{json, Value};
use log::warn;

use crate::config::Config;
use crate::extract_user_info;
use crate::dto::*;
use crate::model::{account::*, user::get_user_info};

pub async fn show_all(
	req: actix_web::HttpRequest,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let _ = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);
	let accounts = match get_all_accounts(db).await{
		Ok(accounts) => accounts,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error": format!("DB Error: {}", e)}));
		}
	};
	HttpResponse::Ok().json(accounts)
}

pub async fn show(
	req: actix_web::HttpRequest,
	params: web::Path<QueryParams>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let _ = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);
	let id = match ObjectId::parse_str(&params.id){
		Ok(id) => id,
		Err(_) => {
			return HttpResponse::BadRequest().json(json!({"error": "Invalid ID"}));
		}
	};
	let account = match get_account(db.clone(), id).await{
		Ok(account) => account,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}));
		}
	};
	match account {
		Some(account) => {
			let creator = get_user_info(account.created_by.clone().unwrap(), db.clone()).await.unwrap();
			let updater = get_user_info(account.updated_by.clone().unwrap(), db).await.unwrap();
			let mut acc_filled = serde_json::to_value(account).unwrap();
			acc_filled["created_by"] = json!(creator.unwrap());
			acc_filled["updated_by"] = json!(updater.unwrap());
			HttpResponse::Ok().json(acc_filled)
		},
		None => HttpResponse::NotFound().json(json!({"error": "Account not found"})),
	}
}

pub async fn create(
	req: actix_web::HttpRequest,
	payload: web::Json<AccountInfo>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let user = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);
	let mut account = payload.clone();
	let id = ObjectId::new();
	account.id = Some(id.clone());
	account.created_by = Some(user.sub.clone());
	account.created_at = Some(DateTime::now());
	account.updated_by = Some(user.sub);
	account.updated_at = Some(DateTime::now());
	match create_account(db, account).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok", "id": id.to_hex()})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}

pub async fn update(
	req: actix_web::HttpRequest,
	payload: web::Json<AccountInfo>,
	conf: web::Data<Config>,
	client: web::Data<Client>,
	param: web::Path<QueryParams>
) -> impl Responder {
	let user = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);

	let id = match ObjectId::parse_str(&param.id){
		Ok(id) => id,
		Err(_) => {
			return HttpResponse::BadRequest().json(json!({"error": "Invalid ID"}));
		}
	};

	let acc = match get_account(db.clone(), id).await{
		Ok(acc) => acc,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}));
		}
	};
	let account = match acc {
		Some(mut account) => {
			account.name = payload.name.clone();
			account.avatar = payload.avatar.clone();
			account.account = payload.account.clone();
			account.password = payload.password.clone();
			account.updated_by = Some(user.sub);
			account.updated_at = Some(DateTime::now());
			account.comment = payload.comment.clone();
			account.auth_id = payload.auth_id.clone();
			account
		},
		None => {
			return HttpResponse::NotFound().json(json!({"error": "Account not found"}));
		}
	};
	match update_account(db, id, account).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}

pub async fn update_auth(
	req: actix_web::HttpRequest,
	payload: web::Json<Value>,
	conf: web::Data<Config>,
	client: web::Data<Client>,
	param: web::Path<QueryParams>
) -> impl Responder {
	let user = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);

	let id = match ObjectId::parse_str(&param.id){
		Ok(id) => id,
		Err(_) => {
			return HttpResponse::BadRequest().json(json!({"error": "Invalid ID"}));
		}
	};

	let acc = match get_account(db.clone(), id).await{
		Ok(acc) => acc,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}));
		}
	};
	let a_id = match payload["auth_id"].as_str(){
		Some(a_id) => Some(ObjectId::parse_str(a_id).unwrap()),
		None => {
			warn!("User {} removed auth from account {}", user.sub, id);
			match delete_auth(db.clone(), id).await{
				Ok(_) => None,
				Err(e) => {
					return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}));
				}
			}
		},
	};
	let account = match acc {
		Some(mut account) => {
			account.auth_id = a_id;
			account.updated_by = Some(user.sub);
			account.updated_at = Some(DateTime::now());
			account
		},
		None => {
			return HttpResponse::NotFound().json(json!({"error": "Account not found"}));
		}
	};
	match update_account(db, id, account).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}

pub async fn delete(
	req: actix_web::HttpRequest,
	params: web::Path<QueryParams>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let user = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	warn!("User {} deleted account {}", user.sub, params.id);
	let db = client.database(&conf.db.db_name);
	let id = match ObjectId::parse_str(&params.id){
		Ok(id) => id,
		Err(_) => {
			return HttpResponse::BadRequest().json(json!({"error": "Invalid ID"}));
		}
	};
	match delete_account(db, id).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}