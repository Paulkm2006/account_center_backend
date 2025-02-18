use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde_json::json;
use serde_json::Value;

use crate::config::Config;
use crate::extract_user_info;
use crate::dto::*;
use crate::model::auth::*;

#[derive(serde::Deserialize)]
pub struct AuthCreateParams{
	pub a_type: String,
}

pub async fn get(
	req: actix_web::HttpRequest,
	params: web::Path<QueryParams>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {
	let _ = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);
	let auth = match get_auth(db, ObjectId::parse_str(&params.id).unwrap()).await{
		Ok(auth) => auth,
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}).to_string());
		}
	};
	match auth {
		Some(auth) => HttpResponse::Ok().json(auth),
		None => HttpResponse::NotFound().json(json!({"error": "Auth not found"}).to_string()),
	}
}

pub async fn create(
	req: actix_web::HttpRequest,
	payload: web::Json<Value>,
	conf: web::Data<Config>,
	client: web::Data<Client>,
	param: web::Path<AuthCreateParams>
) -> impl Responder {
	let _ = extract_user_info!(req, &conf.jwt.secret.as_bytes());
	let db = client.database(&conf.db.db_name);

	match create_auth(db, param.a_type.clone(), payload.clone()).await{
		Ok(id) => HttpResponse::Ok().json(json!({"id": id})),
		Err(e) => HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}).to_string()),
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
	match delete_auth(db, ObjectId::parse_str(&params.id).unwrap()).await{
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => HttpResponse::InternalServerError().json(json!({"error":format!("DB Error: {}", e)}).to_string()),
	}
}