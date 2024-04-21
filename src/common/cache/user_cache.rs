use crate::{
    model::diesel::dolphin::custom_dolphin_models::App, service::app::app_service::query_cached_app,
};
use rust_wheel::{
    config::cache::redis_util::set_str, model::response::user::login_response::LoginResponse,
};

pub fn get_user_cached_key(input_app_id: &String, input_user_id: &i64) -> String {
    let app: App = query_cached_app(&input_app_id);
    return format!("{}{}{}", app.app_abbr, ":user:detail:", input_user_id);
}

pub fn store_login_user(input_app_id: &String, input_user_id: &i64, login_user: &LoginResponse) {
    let u_cached_key = get_user_cached_key(input_app_id, input_user_id);
    let serialized_user = serde_json::to_string(&login_user).unwrap();
    set_str(&u_cached_key, &serialized_user, 36000)
}
