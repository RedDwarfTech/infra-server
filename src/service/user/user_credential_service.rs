use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::custom::user::user_credential_add::UserCredentialAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::UserCredential;

/// 新增一条用户登录凭据记录
pub fn add_user_credential(add: &UserCredentialAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::user_credential as credential_table;
    diesel::insert_into(credential_table::dsl::user_credential)
        .values(add)
        .get_result::<UserCredential>(&mut get_conn())
        .expect("failed to add user credential");
}

/// 按标识（identifier）查询用户登录凭据
///
/// 同一邮箱/标识在不同产品下可能对应不同账号，因此查询需同时
/// 匹配 credential_type 与 product_id，保证按产品维度隔离。
pub fn query_user_credential_by_identifier(
    identifier: &String,
    credential_type: &String,
    filter_product_id: &i32,
) -> Option<UserCredential> {
    use crate::model::diesel::dolphin::dolphin_schema::user_credential as credential_table;
    let predicate = credential_table::identifier
        .eq(identifier)
        .and(credential_table::credential_type.eq(credential_type))
        .and(credential_table::product_id.eq(filter_product_id));
    match credential_table::table
        .filter(&predicate)
        .limit(1)
        .first::<UserCredential>(&mut get_conn())
    {
        Ok(data) => Some(data),
        Err(diesel::result::Error::NotFound) => None,
        Err(err) => {
            log::error!(
                "query user credential facing issue,{},identifier:{},credential_type:{},product_id:{}",
                err,
                identifier,
                credential_type,
                filter_product_id
            );
            None
        }
    }
}
