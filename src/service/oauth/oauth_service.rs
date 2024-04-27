use chrono::Local;
use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::Oauth2RefreshToken;
use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token::expire_date;

pub fn query_refresh_token(input_token: &String) -> Oauth2RefreshToken {
    use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token as oauth_table;
    let predicate = oauth_table::refresh_token.eq(input_token);
    let token = oauth_table::table
        .filter(&predicate)
        .limit(1)
        .first::<Oauth2RefreshToken>(&mut get_conn())
        .expect("query refresh token failed");
    return token;
}

pub fn update_refresh_token_exp_time(db_refresh_token: &Oauth2RefreshToken){
    let now = Local::now();
    let future_time = now + chrono::Duration::days(7);
    let future_timestamp = future_time.timestamp();
    use crate::model::diesel::dolphin::dolphin_schema::oauth2_refresh_token as oauth_table;
    let predicate = oauth_table::id
        .eq(db_refresh_token.id.clone());
    diesel::update(oauth_table::table.filter(predicate))
        .set(expire_date.eq(future_timestamp))
        .get_result::<Oauth2RefreshToken>(&mut get_conn())
        .expect("udpate refresh token expire date failed");
}

