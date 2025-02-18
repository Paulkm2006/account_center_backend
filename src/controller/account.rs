use actix_web::{web, HttpResponse, Responder};
use mongodb::bson;
use mongodb::bson::oid;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde_json::json;
use std::time::SystemTime;

use crate::config::Config;
use crate::extract_user_info;
use crate::dto::*;
use crate::model::account::*;

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
			return HttpResponse::InternalServerError().json(format!("DB Error: {}", e));
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
	let account = match get_account(db, id).await{
		Ok(account) => account,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}));
		}
	};
	match account {
		Some(account) => HttpResponse::Ok().json(account),
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
	let account = AccountInfo{
		id: Some(oid::ObjectId::new()),
		name: payload.name.clone(),
		account: payload.account.clone(),
		password: payload.password.clone(),
		created_by: Some(user.sub.clone()),
		created_at: Some(bson::DateTime::from(SystemTime::now())),
		updated_by: Some(user.sub),
		updated_at: Some(bson::DateTime::from(SystemTime::now())),
		comment: payload.comment.clone(),
		auth_id: payload.auth_id.clone(),
	};
	match create_account(db, account).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}

pub async fn update(
	req: actix_web::HttpRequest,
	payload: web::Json<AccountInfo>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let user = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);

	let id = match payload.id {
		Some(id) => id,
		None => {
			return HttpResponse::BadRequest().json(json!({"error": "id is required"}));
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
			account.account = payload.account.clone();
			account.password = payload.password.clone();
			account.updated_by = Some(user.sub);
			account.updated_at = Some(bson::DateTime::from(SystemTime::now()));
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

pub async fn delete(
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
	match delete_account(db, id).await{
		Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
		Err(e) => {
			HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}))
		}
	}
}