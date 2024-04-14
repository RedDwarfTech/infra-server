use rust_wheel::model::response::user::login_response::LoginResponse;
use crate::model::user::login::login_req::LoginReq;

pub trait RdUser {
    fn login(req: actix_web_validator::Json<LoginReq>) -> LoginResponse;
}


