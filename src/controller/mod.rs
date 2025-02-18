pub mod oauth;
pub mod account;
pub mod auth;

#[macro_export]
macro_rules! extract_user_info {
    ($req:expr, $key:expr) => {{
        let auth_header = match $req.headers().get("Authorization") {
            Some(header) => match header.to_str() {
                Ok(h) => h,
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .json(json!({ "error": "Invalid authorization header" }));
                }
            },
            None => {
                return HttpResponse::Unauthorized()
                    .json(json!({ "error": "Missing authorization header" }));
            }
        };

        match crate::dto::UserInfo::from_header_str($key, auth_header) {
            Ok(user_info) => user_info,
            Err(_) => {
                return HttpResponse::Unauthorized()
                    .json(json!({ "error": "Invalid token" }));
            }
        }
    }};
}
