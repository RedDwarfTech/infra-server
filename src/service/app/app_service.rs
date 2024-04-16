use rust_wheel::config::app::app_conf_reader::get_app_config;
use rust_wheel::config::cache::redis_util::{set_str, sync_get_str};

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::App;

pub fn query_app_by_app_id(filter_app_id: &String) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::apps::app_id.eq(filter_app_id);
    let db_user = apps
        .filter(&predicate)
        .limit(1)
        .first::<App>(&mut get_conn())
        .expect("query user failed");
    return db_user;
}

pub fn query_cached_app(filter_app_id: &String) -> App {
    let stream_key = get_app_config("infra.app_cached_key");
    let cache_key = format!("{}{}",stream_key.as_str(),filter_app_id.as_str());
    let app_str = sync_get_str(&cache_key);
    if app_str.is_none() {
        let db_app = query_app_by_app_id(filter_app_id);
        let serialized_app = serde_json::to_string(&db_app).unwrap();
        set_str(&cache_key,serialized_app.as_str(),360000);
        return db_app;
    }
    let deserialized: App = serde_json::from_str(&app_str.unwrap()).unwrap();
    return deserialized;
}
