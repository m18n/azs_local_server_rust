
mod no_cache_middleware;
mod models;
mod render_temps;
mod check_db_view_middleware;
mod swagger_docs;
mod base;
mod controllers;
mod globals;
mod check_db_api_middleware;


use std::sync::Arc;
use sqlx::{Error as SqlxError, Error, MySql, MySqlPool, Pool, SqlitePool};
use actix_files as fs;
use no_cache_middleware::NoCache;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::migrate::MigrateDatabase;
use tokio::sync::Mutex;
use std::sync;
use crate::controllers::*;
use crate::check_db_view_middleware::CheckDbView;
use crate::models::{AzsDb, get_nowtime_str, local_getMysqlInfo, local_io_getMysqlInfo, local_io_initDb, MyError, MysqlInfo};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::check_db_api_middleware::CheckDbApi;
//use crate::logger::LogManager;
use crate::swagger_docs::ApiDoc;


struct StateDb{
    azs_db:Arc<Mutex<AzsDb>>,
    sqlite:SqlitePool,
}
async fn connect_db(db_url:&str) -> Result<SqlitePool,Error> {

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let db = SqlitePool::connect(&db_url).await?;
    Ok(db)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let logger=LogManager::new().await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test1".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test2".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test3".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test4".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test5".to_string()).await;
    // println!("{}",logger.get_logs_json().await);
    // logger.get_log(vec!["error".to_string()]).await;

    //println!("{}",logger.get_key_json(vec!["error".to_string(),"sqlite".to_string()]).await.to_string());
    let sqlite= connect_db("azs_db.db").await.map_err(|e|

        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    )?;
    println!("Open local database");
    local_io_initDb(&sqlite).await?;
    let mysql_info=local_io_getMysqlInfo(&sqlite).await?;
    let mut azs_db=AzsDb::new();
    let res_conn=azs_db.connect(mysql_info,&sqlite).await;
    match res_conn {
        Ok(_) => {}
        Err(e) => {e.pushlog().await;}
    }
    let state=web::Data::new(StateDb{
        azs_db:Arc::new(Mutex::new(azs_db)),
        sqlite:sqlite,

    });
    println!("START WEB SERVER");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&state))
            .wrap(NoCache)

            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/view")
                .wrap(CheckDbView)
                    .service(view_controller::login)
            )
            .service(fs::Files::new("/public", "./azs_site/public/public").show_files_listing())
            .service(
                web::scope("/settings")
                    .service(api_db_controller::show_error)
                    .service(api_db_controller::show_properties)

            )
            .service(
                web::scope("/api/service")
                    .service(api_controller::check_db_connect)
                    .service(api_controller::set_db_properties)

            )
            .service(
                web::scope("/api/db")
                    .wrap(CheckDbApi)
                        .service(api_controller::test_request)
            )
            .service(
                web::scope("/api/localdb")

            )
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}