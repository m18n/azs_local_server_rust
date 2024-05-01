use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use ramhorns::Template;
use crate::base::file_openString;

use crate::controllers::objects_of_controllers::{AuthInfo, RequestResult};
use crate::globals::LOGS_DB_ERROR;
use crate::jwt::create_token;
use crate::models::{AzsDb, MyError, MysqlInfo, SaveTrksPosition};
use crate::render_temps;
use crate::render_temps::MainTemplate;
use crate::StateDb;
//BASE URL /api/db
#[get("/testDb")]
pub async fn m_test_request(state: web::Data<StateDb>)-> Result<Json<RequestResult>, Error>{
    AzsDb::getUsers(state.azs_db.clone()).await;

    Ok(web::Json(RequestResult {status:true}))
}
#[get("/outshift")]
pub async fn m_out_shift(state: web::Data<StateDb>)-> Result<HttpResponse, Error>{
    AzsDb::closeSmena(state.azs_db.clone()).await?;
    let cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .finish();
    let response = HttpResponse::Found()
        .insert_header((http::header::LOCATION, "/view/login")).cookie(cookie)
        .finish();
    Ok(response)
}
#[post("/auth")]
pub async fn m_auth(auth_info:web::Json<AuthInfo>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{


    let mut is_admin=false;
    let res=AzsDb::checkAuth(state.azs_db.clone(),auth_info.id_user,auth_info.password.clone(),&mut is_admin).await?;

    if res==true {
        AzsDb::setSmenaOperator(state.azs_db.clone(),auth_info.id_user).await?;
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

#[post("/saveTrksPosition")]
pub async fn m_save_trks_position(trks_position:web::Json<SaveTrksPosition>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{

    println!("TRK POSITON: {:?}\n",&trks_position);
    let res=AzsDb::saveTrksPosition(state.azs_db.clone(),trks_position.into_inner()).await?;
    let mut respon = HttpResponse::Ok().json(RequestResult { status: res});
    Ok(respon)
}
