use diesel::{PgConnection, RunQueryDsl};

use crate::model::diesel::{
    custom::pay::payment_add::PaymentAdd, dolphin::custom_dolphin_models::Payment,
};

pub fn save_payment(payments: &PaymentAdd, connection: &mut PgConnection) {
    use crate::model::diesel::dolphin::dolphin_schema::payments as payment_table;
    diesel::insert_into(payment_table::dsl::payments)
        .values(payments)
        .on_conflict_do_nothing()
        .get_result::<Payment>(connection)
        .expect("failed to add new order");
}
