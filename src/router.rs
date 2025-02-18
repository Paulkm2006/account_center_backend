use super::controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/callback")
			.route("", web::get().to(controller::oauth::callback)));
	cfg.service(
		web::scope("/account")
			.route("", web::get().to(controller::account::show_all))
			.route("/{id}", web::get().to(controller::account::show))
			.route("", web::post().to(controller::account::create))
			.route("/{id}", web::put().to(controller::account::update))
			.route("/{id}", web::delete().to(controller::account::delete))
		);
	cfg.service(
		web::scope("/auth")
			.route("/{id}", web::get().to(controller::auth::get))
			.route("/{a_type}", web::post().to(controller::auth::create))
			.route("/{id}", web::delete().to(controller::auth::delete))
		);
	cfg.service(
		web::scope("/user")
			.route("/{id}", web::get().to(controller::user::get))
		);
}