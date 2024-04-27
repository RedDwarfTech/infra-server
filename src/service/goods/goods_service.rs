use rust_wheel::model::enums::rd_deleted_status::RdDeletedStatus;
use rust_wheel::model::enums::rd_online_status::RdOnlineStatus;

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::IapProduct;

pub fn query_goods_list(prod_id: &i32) -> Vec<IapProduct> {
    use crate::model::diesel::dolphin::dolphin_schema::iap_product as prod_table;
    let predicate = prod_table::product_id.eq(prod_id.clone())
    .and(prod_table::online_status.eq(RdOnlineStatus::Online as i32))
    .and(prod_table::deleted.eq(RdDeletedStatus::Normal as i32));
    let err_msg = format!("{}{}","query by product id failed,prod_id:", prod_id);
    let products = prod_table::table
        .filter(&predicate)
        .load::<IapProduct>(&mut get_conn())
        .expect(&err_msg);
    return products;
}

pub fn query_goods_by_id(prod_id: &i64) -> IapProduct {
    use crate::model::diesel::dolphin::dolphin_schema::iap_product as prod_table;
    let predicate = prod_table::id.eq(prod_id.clone());
    let err_msg = format!("{}{}","query by product id failed,prod_id:", prod_id);
    let product = prod_table::table
        .filter(&predicate)
        .first::<IapProduct>(&mut get_conn())
        .expect(&err_msg);
    return product;
}