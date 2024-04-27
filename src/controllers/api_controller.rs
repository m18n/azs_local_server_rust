use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use ramhorns::Template;
use serde::Serialize;
use crate::base::file_openString;
use crate::globals::LOGS_DB_ERROR;
use crate::models::{DbStatus, MyError, MysqlInfo, TypesStatus};
use crate::render_temps::{ErrorDb, MysqlInfowithErrorDb};
use crate::StateDb;

#[derive(Serialize)]
struct DbWriteResult{
    status:bool,
}
//BASE URL /api
#[get("/checkDbConnection")]
pub async fn  check_db_connect(state: web::Data<StateDb>)-> Result<Json<DbStatus>, Error>
{
    let mut azs_db=state.azs_db.lock().await;

    Ok(web::Json(azs_db.getDbStatus()))

}
#[post("/setDbProperties")]
pub async fn  set_db_properties(mysql_info:web::Json<MysqlInfo>,state: web::Data<StateDb>)-> Result<Json<DbWriteResult>, Error>
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

  Ok(web::Json(DbWriteResult{status:true}))
}
#[get("/testDb")]
pub async fn test_request(state: web::Data<StateDb>)-> Result<Json<DbWriteResult>, Error>{

    let mut azs_db=state.azs_db.lock().await;
    azs_db.getUsers().await?;
    Ok(web::Json(DbWriteResult{status:true}))
}