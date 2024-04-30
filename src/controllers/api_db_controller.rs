use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use ramhorns::Template;
use crate::base::file_openString;

use crate::controllers::objects_of_controllers::{AuthInfo,  RequestResult};
use crate::globals::LOGS_DB_ERROR;
use crate::jwt::create_token;
use crate::models::{AzsDb, MysqlInfo};
use crate::render_temps;
use crate::StateDb;
//BASE URL /api/db
#[get("/testDb")]
pub async fn m_test_request(state: web::Data<StateDb>)-> Result<Json<RequestResult>, Error>{
    AzsDb::getUsers(state.azs_db.clone()).await;

    Ok(web::Json(RequestResult {status:true}))
}
#[post("/auth")]
pub async fn m_auth(auth_info:web::Json<AuthInfo>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{


    let mut is_admin=false;
    let res=AzsDb::checkAuth(state.azs_db.clone(),auth_info.id_user,auth_info.password.clone(),&mut is_admin).await?;

    if res==true {
        let cookie = Cookie::build("refresh_token", create_token(auth_info.id_user, is_admin))
            .path("/")
            .http_only(true)
            .finish();
        let mut respon = HttpResponse::Ok().cookie(cookie).json(RequestResult { status: res });
        Ok(respon)
    }else{
        let mut respon = HttpResponse::Ok().json(RequestResult { status: res });
        Ok(respon)
    }

}
