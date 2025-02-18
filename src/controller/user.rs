use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use serde_json::json;


use crate::config::Config;
use crate::dto::QueryParams;
use crate::extract_user_info;
use crate::model::user::get_user_info;

pub async fn get(
	req: actix_web::HttpRequest,
	conf: web::Data<Config>,
	client: web::Data<Client>,
	param: web::Path<QueryParams>,
) -> impl Responder {
	let _ = extract_user_info!(req, conf.jwt.secret.as_bytes());
	let db = client.database(conf.db.db_name.as_str());
	let user = get_user_info(param.id.clone(), db).await;
	match user {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(e) => HttpResponse::InternalServerError().json(json!({"error": format!("DB Error: {}", e)}).to_string()),
	}
}