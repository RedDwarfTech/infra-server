use diesel::{PgConnection, RunQueryDsl};
use log::error;
use crate::diesel::OptionalExtension;

use crate::model::diesel::{
    custom::pay::payment_add::PaymentAdd, dolphin::custom_dolphin_models::Payment,
};

pub fn save_payment(payments: &PaymentAdd, connection: &mut PgConnection) {
    use crate::model::diesel::dolphin::dolphin_schema::payments as payment_table;
    // https://github.com/diesel-rs/diesel/issues/952
    let insert_result = diesel::insert_into(payment_table::dsl::payments)
        .values(payments)
        .on_conflict_do_nothing()
        .get_result::<Payment>(connection)
        .optional();
    if let Err(err) = insert_result {
        error!("insert payments facing issue: {}", err);
    }
}
