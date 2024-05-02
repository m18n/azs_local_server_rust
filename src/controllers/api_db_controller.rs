use std::alloc::alloc;
use std::future::Future;
use std::pin::Pin;
use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use futures_util::future::join_all;
use futures_util::stream::All;
use ramhorns::Template;
use serde_json::from_value;
use sqlx::Value;
use crate::base::file_openString;

use crate::controllers::objects_of_controllers::{AllObject, AllObject_ID, AuthInfo, RequestResult};
use crate::globals::LOGS_DB_ERROR;
use crate::jwt::create_token;
use crate::models::{AzsDb, BoxedFutureBool, MyError, MysqlInfo, SaveTrksPosition};
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
        let azs_db=state.azs_db.lock().await;
        let cookie = Cookie::build("refresh_token", create_token(auth_info.id_user, is_admin,azs_db.mysql_info_success.clone()))
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
#[get("/settings/get")]
pub async fn m_settings_get(state: web::Data<StateDb>)-> Result<HttpResponse, Error>{
    let ( tovars_result, tanks_result, trks_result) = tokio::join!(
        AzsDb::getTovars(state.azs_db.clone()), AzsDb::getTanks(state.azs_db.clone()), AzsDb::getTrks(state.azs_db.clone()));
    let tovars = tovars_result?;
    let tanks = tanks_result?;
    let trks = trks_result?;
    let mut respon = HttpResponse::Ok().json(AllObject{trks:Some(trks),tovars:Some(tovars),tanks:Some(tanks)});
    Ok(respon)
}


#[post("/settings/set")]
pub async fn m_settings_set(all_object:web::Json<AllObject>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{
    let all_object=all_object.into_inner();
    let mut vector_tasks: Vec<BoxedFutureBool>=Vec::new();
    if let Some(tovars) = all_object.tovars {
        vector_tasks.push(Box::pin(AzsDb::setTovars(state.azs_db.clone(), tovars)));
    }
    if let Some(trks) = all_object.trks {
        vector_tasks.push(Box::pin(AzsDb::setTrks(state.azs_db.clone(), trks)));
    }
    if let Some(tanks) = all_object.tanks {
        vector_tasks.push(Box::pin(AzsDb::setTanks(state.azs_db.clone(), tanks)));
    }

    let results = join_all(vector_tasks).await;
    for res in results {
        res?;
    }

    let mut respon = HttpResponse::Ok().json(RequestResult { status: true});
    Ok(respon)
}
#[post("/settings/delete")]
pub async fn m_settings_delete(all_object:web::Json<AllObject_ID>,state: web::Data<StateDb>)-> Result<HttpResponse, Error>{
    let all_object=all_object.into_inner();
    let mut vector_tasks: Vec<BoxedFutureBool>=Vec::new();
    if let Some(tovars) = all_object.tovars {
        //vector_tasks.push(Box::pin(AzsDb::setTovars(state.azs_db.clone(), tovars)));
    }
    if let Some(trks) = all_object.trks {
        vector_tasks.push(Box::pin(AzsDb::deleteTrks(state.azs_db.clone(), trks)));
    }
    if let Some(tanks) = all_object.tanks {
        //vector_tasks.push(Box::pin(AzsDb::setTanks(state.azs_db.clone(), tanks)));
    }

    let results = join_all(vector_tasks).await;
    for res in results {
        res?;
    }

    let mut respon = HttpResponse::Ok().json(RequestResult { status: true});
    Ok(respon)
}