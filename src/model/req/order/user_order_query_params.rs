#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
pub struct UserOrderQueryParams {
    pub pageNum: Option<i64>,
    pub pageSize: Option<i64>,
}