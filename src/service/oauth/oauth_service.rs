use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::custom::oauth::oauth_add::OauthAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::Oauth2RefreshToken;
use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token::expire_date;
use chrono::Local;
use log::error;

pub fn query_refresh_token(input_token: &String) -> Option<Oauth2RefreshToken> {
    use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token as oauth_table;
    let predicate = oauth_table::refresh_token.eq(input_token);
    let err_msg = format!("query refresh token failed，input token:{}", input_token);
    let result = oauth_table::table
        .filter(&predicate)
        .limit(1)
        .first::<Oauth2RefreshToken>(&mut get_conn());
    match result {
        Ok(data) => {
            return Some(data);
        }
        Err(_) => {
            error!("{}",err_msg);
            return None;
        }
    }
}

pub fn insert_refresh_token(oauth_new: &OauthAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token as oauth_table;
    diesel::insert_into(oauth_table::dsl::oauth2_refresh_token)
        .values(oauth_new)
        .get_result::<Oauth2RefreshToken>(&mut get_conn())
        .expect("failed to add new refresh token");
}

pub fn update_refresh_token_exp_time(db_refresh_token: &Oauth2RefreshToken) {
    let now = Local::now();
    let future_time = now + chrono::Duration::days(7);
    let future_timestamp = future_time.timestamp();
    use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token as oauth_table;
    let predicate = oauth_table::id.eq(db_refresh_token.id.clone());
    diesel::update(oauth_table::table.filter(predicate))
        .set(expire_date.eq(future_timestamp))
        .get_result::<Oauth2RefreshToken>(&mut get_conn())
        .expect("udpate refresh token expire date failed");
}
