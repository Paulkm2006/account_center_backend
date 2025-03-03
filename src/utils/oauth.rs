use actix_web::web;
use anyhow::Context;
use base64::Engine as _;
use serde_json::Value;

use crate::config::Config;
use crate::dto::UserInfo;



pub async fn get_info(code: &str, conf: web::Data<Config>) -> Result<UserInfo, Box<dyn std::error::Error>> {
	let http_client = reqwest::ClientBuilder::new()
	.redirect(reqwest::redirect::Policy::none())
	.build()
	.context("Client should build")?;

	let token_response = http_client.post(&conf.oidc.token_url)
		.form(
			&[
				("grant_type", "authorization_code"),
				("code", &code),
				("redirect_uri", &conf.oidc.redirect_uri),
				("client_id", &conf.oidc.client_id),
				("client_secret", &conf.oidc.client_secret),
			])
		.send()
		.await.context("Request should be successful")?.text().await.context("Response should be text")?;

	let claims: Value = serde_json::from_str(&token_response).context("Claims should be json encoded")?;
	let claims = claims["id_token"].as_str().context("Token should have id_token")?;
	let claims = claims.split('.').nth(1).context("Token should have 3 parts")?.replace('-', "+").replace('_', "/");
	let claims = base64::engine::general_purpose::STANDARD_NO_PAD.decode(claims).context("Claims should be base64 encoded")?;
	let claims = String::from_utf8(claims).context("Claims should be utf8 encoded")?;
	let mut claim: UserInfo = serde_json::from_str(&claims).context("Binding claims failed")?;

	if claim.picture.is_none() {
		claim.picture = Some("https://gravatar.com/avatar/".to_string() + &sha256::digest(claim.email.as_bytes()));
	}

	Ok(claim)
}