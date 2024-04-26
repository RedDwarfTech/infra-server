use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::IapProduct;

pub fn query_goods_list(prod_id: &i32) -> Vec<IapProduct> {
    use crate::model::diesel::dolphin::dolphin_schema::iap_product as prod_table;
    let predicate = prod_table::product_id.eq(prod_id.clone());
    let err_msg = format!("{}{}","query by product id failed,prod_id:", prod_id);
    let products = prod_table::table
        .filter(&predicate)
        .limit(1)
        .load::<IapProduct>(&mut get_conn())
        .expect(&err_msg);
    return products;
}
