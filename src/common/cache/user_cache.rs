use crate::{
    model::diesel::dolphin::custom_dolphin_models::{App, User},
    service::app::app_service::query_cached_app,
};
use rust_wheel::{
    config::cache::redis_util::set_str,
    model::user::{rd_user_info::RdUserInfo, web_jwt_payload::WebJwtPayload},
};

pub fn get_rd_user_cached_key(input_app_id: &String, input_user_id: &i64) -> String {
    let app: App = query_cached_app(&input_app_id);
    return format!("{}{}{}", app.app_abbr, ":user:detail:", input_user_id);
}

pub fn get_user_cached_key(input_app_id: &String, input_user_id: &i64) -> String {
    let app: App = query_cached_app(&input_app_id);
    return format!("{}{}{}", app.app_abbr, ":db-user:detail:", input_user_id);
}

pub fn get_user_by_phone_cached_key(input_app_id: &String, phone_number: &String) -> String {
    let app: App = query_cached_app(&input_app_id);
    return format!("{}{}{}", app.app_abbr, ":db-user:detail:", phone_number);
}

pub fn store_login_user(payload: &WebJwtPayload, login_user: &User, app_info: &App) {
    let u_cached_key = get_rd_user_cached_key(&payload.appId, &payload.userId);
    let rd_user = RdUserInfo {
        id: payload.userId,
        nickname: login_user.nickname.to_string(),
        device_id: payload.deviceId.to_string(),
        app_id: payload.appId.to_string(),
        avatar_url: login_user.avatar_url.clone(),
        auto_renew_product_expire_time_ms: 0,
        app_name: app_info.app_name.to_string(),
        salt: login_user.salt.to_string(),
    };

    let serialized_user = serde_json::to_string(&rd_user).unwrap();
    set_str(&u_cached_key, &serialized_user, 36000)
}
