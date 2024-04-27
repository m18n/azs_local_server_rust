use std::cmp::PartialEq;
use std::sync::Arc;
use actix_web::{HttpResponse, ResponseError, web};
use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySqlPool, SqlitePool};
use sqlx::FromRow;
use crate::StateDb;
use thiserror::Error;
use crate::base::file_openString;
use chrono::{Local, Datelike, Timelike};
use http::StatusCode;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use serde::de::Unexpected::Str;
use crate::globals::LOGS_DB_ERROR;

pub fn get_nowtime_str()->String{
    let current_datetime = Local::now();

    // Отримуємо значення року, місяця, дня, години та хвилини
    let year = current_datetime.year();
    let month = current_datetime.month();
    let day = current_datetime.day();
    let hour = current_datetime.hour();
    let minute = current_datetime.minute();

    // Складаємо значення у рядок
    let datetime_string = format!("{}-{:02}-{:02} {:02}:{:02}", year, month, day, hour, minute);
    datetime_string

}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("")]
    DatabaseError(String), // Автоматично конвертує sqlx::Error у MyError
    // Додайте інші варіанти помилок тут
}
impl MyError{
    pub async fn pushlog(&self){
        match self {
            MyError::DatabaseError(mess_err) => {
                let mess_err = mess_err.clone();
                let mut log = LOGS_DB_ERROR.lock().await;
                log.push_str(&mess_err);
            }
        }
    }
}
impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {

        return StatusCode::BAD_REQUEST;
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::DatabaseError(mess_err) => {
                let mess_err = mess_err.clone();
                tokio::spawn(async move{
                    let mut log = LOGS_DB_ERROR.lock().await;
                    log.push_str(&mess_err);
                });

                HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/settings/dberror"))
                .finish()
            }

            // Обробіть інші варіанти помилок тут
        }
    }
}
#[derive(Debug, Serialize, Deserialize, FromRow,Content)]
pub struct User{
    id_user:i32,
    #[sqlx(rename = "user")]
    name:String
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,Content,PartialEq)]
pub struct MysqlInfo{
    pub ip:String,
    pub login:String,
    pub password:String,
    pub database:String,
    pub port:String
}

impl MysqlInfo {
    pub fn new()->MysqlInfo{
        MysqlInfo{ip:String::new(),login:String::new(),password:String::new(),database:String::new(),port:String::new()}
    }
    pub fn is_empty(&self)->bool{
        if self.ip==""&&self.login==""&&self.password==""&&self.database==""&&self.port=="" {
            true
        }
        else {
            false
        }
    }
}
#[derive(Serialize)]
pub enum TypesStatus {
    Connected,
    Disconnected,
    Connecting,
}
#[derive(Serialize)]
pub struct DbStatus{
    pub status:TypesStatus
}

pub struct AzsDb{
    pub mysql:Option<MySqlPool>,
    pub mysql_info_success:MysqlInfo,
    pub mysql_info_last:MysqlInfo,
    pub is_connecting:bool,

}


impl AzsDb {
    pub fn new()->AzsDb{
        AzsDb{mysql:None,mysql_info_success:MysqlInfo::new(),mysql_info_last:MysqlInfo::new(),is_connecting:false}
    }
    pub async fn disconnect(&mut self){
        self.is_connecting=false;
        self.mysql=None;
    }
    pub async fn connect(&mut self,mysql_info:MysqlInfo,sqlite_pool: &SqlitePool)->Result<bool, MyError>{
        let database_url = format!("mysql://{}:{}@{}:{}/{}",mysql_info.login,mysql_info.password,mysql_info.ip,mysql_info.port,mysql_info.database);
        println!("CONNECT INFO: {}",database_url);
        let mut mysql_info_success=MysqlInfo::new();
        let mut mysql_info_lats=MysqlInfo::new();

        self.mysql=None;
        self.is_connecting=true;
        self.mysql_info_last=mysql_info.clone();
        self.mysql=match MySqlPool::connect(&database_url).await{
            Ok(pool)=>{
                println!("CONNECTION to mysql db successfully");
                if(self.mysql_info_success!=mysql_info){
                    local_setMysqlInfo(sqlite_pool, mysql_info.clone()).await?;
                }
                let mut log = LOGS_DB_ERROR.lock().await;
                log.clear();
                self.mysql_info_success=mysql_info.clone();

                Some(pool)},
            Err(e)=>{
                self.disconnect().await;
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                return Err(MyError::DatabaseError(str_error))
            },
        };

        self.is_connecting=false;
        Ok(!self.mysql.is_none())
    }
    pub async fn getUsers(&mut self)-> Result<Vec<User>, MyError> {

        let users= sqlx::query_as("SELECT * FROM loc_users INNER JOIN ref_users ON loc_users.id_user = ref_users.id_user;")
            .fetch_all(self.mysql.as_ref().unwrap())
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
        Ok(users)
    }
    pub fn getDbStatus(&self)->DbStatus{
        if(self.mysql.is_none()){
            if self.is_connecting==true{
                DbStatus{status:TypesStatus::Connecting}
            }else{
                DbStatus{status:TypesStatus::Disconnected}
            }
        }else{
           DbStatus{status:TypesStatus::Connected}
        }

    }
}
pub async fn local_io_initDb(sqlite:&SqlitePool)->Result<bool,std::io::Error>{
    let res = sqlx::query("CREATE TABLE IF NOT EXISTS mysql_db (
                          id INTEGER PRIMARY KEY,
                          ip TEXT NOT NULL,
                          login TEXT NOT NULL,
                          password TEXT NOT NULL,
                          database TEXT NOT NULL,
                          port TEXT NOT NULL
                          );")
        .execute(sqlite)
        .await;
    match res {
        Ok(_) =>  Ok(true),
        Err(e) =>{
            let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());
            tokio::spawn(async move {
                let mut log = LOGS_DB_ERROR.lock().await;
                log.push_str(&str_error);
            });
            Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
    }
}
pub async fn local_io_getMysqlInfo(sqlite:&SqlitePool)->Result<MysqlInfo,std::io::Error>{
    let mysql_info= sqlx::query_as::<_,MysqlInfo>("SELECT * FROM mysql_db;")
        .fetch_all(sqlite)
        .await;
    match mysql_info {
        Ok(mysql_info) =>  {
            if mysql_info.len()!=0 {
                Ok(mysql_info[0].clone())
            }
            else{
                Ok(MysqlInfo::new())
            }
        },
        Err(e) =>{
            let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());
            tokio::spawn(async move {
                let mut log = LOGS_DB_ERROR.lock().await;
                log.push_str(&str_error);
            });
            Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        },
    }

}
pub async fn local_getMysqlInfo(sqlite:&SqlitePool)->Result<MysqlInfo, MyError>{
    let mysql_info= sqlx::query_as::<_,MysqlInfo>("SELECT * FROM mysql_db;")
        .fetch_all(sqlite)
        .await.map_err( |e|  {
            let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());
            MyError::DatabaseError(str_error)
        })?;
    if mysql_info.len()!=0 {
        Ok(mysql_info[0].clone())
    }
    else{
        Ok(MysqlInfo::new())
    }
}
pub async fn local_setMysqlInfo(sqlite:&SqlitePool,mysqlinfo:MysqlInfo)->Result<bool, MyError>{
    let row_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mysql_db")
        .fetch_one(sqlite)
        .await.map_err( |e|  {
        let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());
        MyError::DatabaseError(str_error)
    })?;
    if row_count==0{
        sqlx::query("INSERT INTO mysql_db (ip, login, password, database, port) VALUES (?, ?, ?, ?, ?)")
            .bind(mysqlinfo.ip)
            .bind(mysqlinfo.login)
            .bind(mysqlinfo.password)
            .bind(mysqlinfo.database)
            .bind(mysqlinfo.port)
            .execute(sqlite)
            .await.map_err( |e|  {
                let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
    }else{
        sqlx::query("UPDATE mysql_db SET ip=?, login=?, password=?, database=?, port=? WHERE id = (SELECT id FROM mysql_db LIMIT 1)")
            .bind(mysqlinfo.ip)
            .bind(mysqlinfo.login)
            .bind(mysqlinfo.password)
            .bind(mysqlinfo.database)
            .bind(mysqlinfo.port)
            .execute(sqlite)
            .await.map_err( |e|  {
                let str_error = format!("SQLITE|| {} error: {}\n", get_nowtime_str(), e.to_string());


                MyError::DatabaseError(str_error)
            })?;
    }
    Ok(true)
}
