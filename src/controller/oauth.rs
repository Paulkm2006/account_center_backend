use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use serde::Deserialize;
use serde_json::json;


use crate::config::Config;
use crate::utils::oauth::get_info;

#[derive(Deserialize)]
pub struct OidcCallbackParams {
	pub code: String,
}


pub async fn callback(
	params: web::Query<OidcCallbackParams>,
	conf: web::Data<Config>,
	client: web::Data<Client>
) -> impl Responder {

	let code = params.code.clone();

	let user = match get_info(&code, conf.clone()).await{
		Ok(user) => user,
		Err(e) => {
			return HttpResponse::Unauthorized().json(json!({"error": e.to_string()}));
		}
	};

	let db = client.database(conf.db.db_name.as_str());

	match crate::model::user::check_user(&user, db).await{
		Ok(_) => (),
		Err(e) => {
			return HttpResponse::InternalServerError().json(json!({"error": e.to_string()}));
		}
	};

	let token = crate::utils::jwt::new_token(user.clone(), &conf.jwt.secret.as_bytes(), conf.jwt.expire);
	HttpResponse::Ok().json(json!({"token": token}))
}