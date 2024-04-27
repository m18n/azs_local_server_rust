use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use ramhorns::Template;
use crate::base::file_openString;

use crate::controllers::objects_of_controllers::{AuthInfo,  RequestResult};
use crate::globals::LOGS_DB_ERROR;
use crate::jwt::create_token;
use crate::models::MysqlInfo;
use crate::render_temps;
use crate::StateDb;
//BASE URL /api/db
#[get("/testDb")]
pub async fn test_request(state: web::Data<StateDb>)-> Result<Json<RequestResult>, Error>{

    let mut azs_db=state.azs_db.lock().await;
    azs_db.getUsers().await?;
    Ok(web::Json(RequestResult {status:true}))
}
#[post("/auth")]
pub async fn auth(auth_info:web::Json<AuthInfo>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{

    let mut azs_db=state.azs_db.lock().await;
    let mut is_admin=false;
    let res=azs_db.checkAuth(auth_info.id_user,auth_info.password.clone(),&mut is_admin).await?;

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
