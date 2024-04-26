use actix_web::HttpResponse;
use crate::model::req::user::login::login_req::LoginReq;

pub trait RdUser {
    fn login(req: actix_web_validator::Json<LoginReq>) -> HttpResponse;
}


