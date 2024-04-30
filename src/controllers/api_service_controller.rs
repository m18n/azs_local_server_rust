use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use ramhorns::Template;
use serde::Serialize;
use crate::base::file_openString;
use crate::controllers::objects_of_controllers::{ RequestResult};
use crate::globals::LOGS_DB_ERROR;
use crate::models::{DbStatus, MyError, MysqlInfo, TypesStatus};
use crate::render_temps::{ErrorDb, MysqlInfowithErrorDb};
use crate::StateDb;


//BASE URL /api
#[get("/checkDbConnection")]
pub async fn  m_check_db_connect(state: web::Data<StateDb>)-> Result<Json<DbStatus>, Error>
{
    let mut azs_db=state.azs_db.lock().await;

    Ok(web::Json(azs_db.getDbStatus()))

}
#[post("/setDbProperties")]
pub async fn  m_set_db_properties(mysql_info:web::Json<MysqlInfo>,state: web::Data<StateDb>)-> Result<Json<RequestResult>, Error>
{
   println!("{}",serde_json::to_string(&mysql_info).unwrap());
    tokio::spawn(async move {
        let mut azs_db=state.azs_db.lock().await;
        let res=azs_db.connect(mysql_info.into_inner(),&state.sqlite).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                e.pushlog().await;
            }
        }

    });

  Ok(web::Json(RequestResult{status:true}))
}
