use super::rd_user::RdUser;
use crate::model::diesel::dolphin::custom_dolphin_models::User;
use crate::service::app::app_service::query_app_by_app_id;
use crate::{
    model::user::login::login_req::LoginReq, service::user::user_service::query_user_by_product_id,
};
use actix_web::{web, HttpResponse, Responder};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use rust_wheel::common::wrapper::actix_http_resp::box_error_actix_rest_response;
use rust_wheel::model::response::user::login_response::LoginResponse;
use rust_wheel::model::user::jwt_auth::create_access_token;
use rust_wheel::model::user::rd_user_info::RdUserInfo;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FileQueryParams {
    pub file_id: String,
}

struct UserController {}

impl RdUser for UserController {
    fn login(req: actix_web_validator::Json<LoginReq>) -> HttpResponse {
        let app_info = query_app_by_app_id(&req.0.app_id);
        let single_user: User = query_user_by_product_id(&req.0, &app_info.product_id);
        let pwd_salt = single_user.salt;
        let sha_password = get_sha(String::from(&req.password), &pwd_salt);
        if sha_password.eq(&single_user.pwd.as_str()) {
            let rd_user = RdUserInfo{
                id: single_user.id,
                nickname: single_user.nickname.to_string(),
                device_id: req.0.device_id,
                app_id: req.0.app_id,
            };
            let uuid = Uuid::new_v4();
            let access_token = create_access_token(&rd_user);
            let login_resp = LoginResponse{ 
                registerTime: single_user.register_time, 
                refreshToken: uuid.to_string(), 
                accessToken: access_token,
                nickname: single_user.nickname.to_string()
            };
            return box_actix_rest_response(login_resp);
        } else {
            return box_error_actix_rest_response("LOGIN_INFO_NOT_MATCH", "0030010001".to_owned(), "登录信息不匹配".to_owned());
        }
    }
}

pub async fn get_file(_params: web::Query<FileQueryParams>) -> impl Responder {
    box_actix_rest_response("ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/infra/user").route("/list", web::get().to(get_file)));
}
