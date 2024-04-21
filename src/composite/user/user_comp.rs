use crate::{
    common::cache::user_cache::get_user_cached_key,
    model::diesel::dolphin::custom_dolphin_models::App,
    service::{app::app_service::query_cached_app, user::user_service::query_user_by_id},
};
use rust_wheel::{
    config::cache::redis_util::sync_get_str,
    model::user::{login_user_info::LoginUserInfo, rd_user_info::RdUserInfo},
};

pub fn comp_current_user(login_user_info: &LoginUserInfo) -> RdUserInfo {
    let app: App = query_cached_app(&login_user_info.appId);
    let current_u = get_cached_user(login_user_info, &app);
    return current_u;
}

pub fn get_cached_user(login_user_info: &LoginUserInfo, app: &App) -> RdUserInfo {
    let user_cached_key = get_user_cached_key(&app.app_abbr, &login_user_info.userId);
    let cached_user_info = sync_get_str(&user_cached_key);
    if cached_user_info.is_some() {
        let u_model: RdUserInfo = serde_json::from_str(&cached_user_info.unwrap()).unwrap();
        return u_model;
    }
    let u_info = query_user_by_id(&login_user_info.userId);
    let rd_user = RdUserInfo {
        id: u_info.id,
        nickname: u_info.nickname,
        device_id: login_user_info.deviceId.to_string(),
        app_id: u_info.app_id,
        avatar_url: u_info.avatar_url.unwrap_or_default(),
        auto_renew_product_expire_time_ms: 0,
        app_name: app.app_name.to_string(),
    };
    return rd_user;
}
