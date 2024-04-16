use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::Oauth2RefreshToken;

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
