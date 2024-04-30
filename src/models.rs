use std::cmp::PartialEq;
use std::ffi::c_double;
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

use sqlx::mysql::MySqlQueryResult;
use crate::globals::LOGS_DB_ERROR;
use serde_json::Value::String as JsonString;
use std::string::String;
use futures_util::future::join_all;

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
    name:String,
    admin:bool
}
#[derive(Debug, Serialize, Deserialize, FromRow,Content)]
pub struct UserDb{
    id_user:i32,
    #[sqlx(rename = "user")]
    name:String,
}
#[derive(Debug, Serialize, Deserialize, FromRow,Content,Clone)]
pub struct ScreenSize{
    pub width:i32,
    pub height:i32,
}
#[derive(Debug, Serialize, Deserialize, FromRow,Content)]
pub struct Tank{
    pub id_tank:i32,
    pub id_tovar:i32,
    pub volume:i32,
    pub remain:i32,
}
#[derive(sqlx::FromRow, Debug,Serialize, Deserialize)]
pub struct TrkDb {
    id_trk: i32,
    x_pos: i32,
    y_pos: i32,
    scale: f64,
    id_pist: i32,
    id_tank: i32,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Pist {
    pub id_pist: i32,
    pub id_tank: i32,
}
#[derive(sqlx::FromRow, Debug,Serialize, Deserialize)]
pub struct Trk {
    pub nn:i32,
    pub id_trk: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub scale: f64,
    pub pists: Vec<Pist>,
}
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct PositionTrk{
    pub id:i32,
    pub x:f32,
    pub y:f32,
    pub scale:f32,
}
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct SaveTrksPosition{
    screen_scale:ScreenSize,
    objects: Vec<PositionTrk>
}
#[derive(sqlx::FromRow, Debug,Serialize, Deserialize)]
pub struct TovarDb {
    id_tovar:i32,
    price: f32,
    name: String,
    name_p: String,
    name_p_f: String,
    name_p_v:String,
    color:i32,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Color{
    pub r:u8,
    pub g:u8,
    pub b:u8
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Tovar {
    pub id_tovar:i32,
    pub price: f32,
    pub name: String,
    pub name_p: String,
    pub name_p_f: String,
    pub name_p_v:String,
    pub color:Color
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
    pub async fn getUsers(azs_db:Arc<Mutex<AzsDb>>)-> Result<Vec<User>, MyError> {
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let users_db:Vec<UserDb>= sqlx::query_as("SELECT * FROM loc_users INNER JOIN ref_users ON loc_users.id_user = ref_users.id_user;")
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
        let mut users:Vec<User>=Vec::with_capacity(users_db.len());
        for item in users_db{
            let mut user_ =User{id_user:item.id_user,name:item.name,admin:false};
            if user_.id_user >= 1000000{
                user_.admin=true;
            }
            users.push(user_);
        }
        Ok(users)
    }
    pub async fn getTanks(azs_db:Arc<Mutex<AzsDb>>)-> Result<Vec<Tank>, MyError> {
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let tanks:Vec<Tank>= sqlx::query_as("SELECT * FROM tank ORDER BY NN;")
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;

        Ok(tanks)
    }
    pub async fn getTovars(azs_db:Arc<Mutex<AzsDb>>)->Result<Vec<Tovar>,MyError>{
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let query = r#"
            SELECT * FROM tovar ORDER BY NN;
        "#;

        let tovars_db:Vec<TovarDb>= sqlx::query_as(query)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
        let mut tovars=Vec::with_capacity(tovars_db.len());
        if tovars_db.is_empty()
        {
            return Ok(tovars);
        }

        for item in tovars_db{
            let red = ((item.color >> 16) & 0xFF) as u8;
            let green = ((item.color >> 8) & 0xFF) as u8;
            let blue = (item.color & 0xFF) as u8;
            tovars.push(Tovar{id_tovar:item.id_tovar,price:item.price,name:item.name,name_p:item.name_p,name_p_f:item.name_p_f,name_p_v:item.name_p_v,
            color:Color{r:red,g:green,b:blue}});
        }
        Ok(tovars)
    }
    pub async fn getTrks(azs_db:Arc<Mutex<AzsDb>>)-> Result<Vec<Trk>, MyError> {
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let query = r#"
            SELECT com_trk.id_trk, com_trk.x_pos, com_trk.y_pos, com_trk.scale,
                   trk.id_trk as trk_id_trk, trk.id_pist, trk.id_tank
            FROM com_trk
            INNER JOIN trk ON com_trk.id_trk = trk.id_trk
            ORDER BY com_trk.id_trk;
        "#;

        let trks_db:Vec<TrkDb>= sqlx::query_as(query)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;

        let mut trks=Vec::with_capacity(trks_db.len());
        if trks_db.is_empty()
        {
            return Ok(trks);
        }
        let mut last_id:i32=-1;
        let mut n=0;

        for item in trks_db{

            if(last_id!=item.id_trk) {
                n += 1;
                trks.push(Trk { nn: n, id_trk: item.id_trk, x_pos: item.x_pos, y_pos: item.y_pos, scale: item.scale, pists: Vec::new() });
            }
            trks.last_mut().unwrap().pists.push(Pist{id_pist:item.id_pist,id_tank:item.id_tank});
            last_id=item.id_trk;
        }
        Ok(trks)
    }
    pub async fn saveTrkPosition(azs_db:Arc<Mutex<AzsDb>>,trk_pos:PositionTrk,screen_size: ScreenSize)->Result<bool, MyError>{
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let screen_width_f=screen_size.width as f32;
        let x_pos_f=screen_width_f/(100.0/trk_pos.x);
        let y_pos_f=screen_width_f/(100.0/trk_pos.y);
        let x_pos:i32=x_pos_f as i32;
        let y_pos:i32=y_pos_f as i32;
        let users= sqlx::query("UPDATE com_trk SET scale=?, x_pos=?, y_pos=? WHERE id_trk=?;")
            .bind(trk_pos.scale)
            .bind(x_pos)
            .bind(y_pos)
            .bind(trk_pos.id)
            .execute(&mysqlpool)
            .await;
        match users {
            Ok(_) => {
                Ok(true)
            }
            Err(e) => {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                Err(MyError::DatabaseError(str_error))
            }
        }
    }
    pub async fn saveTrksPosition(azs_db:Arc<Mutex<AzsDb>>,trks_pos:SaveTrksPosition)->Result<bool, MyError>{
        Self::setScreenSize(azs_db.clone(),trks_pos.screen_scale.clone()).await?;
        let mut vector_tasks =Vec::new();
        for element in trks_pos.objects{
            vector_tasks.push(Self::saveTrkPosition(azs_db.clone(),element.clone(),trks_pos.screen_scale.clone()));
        }
        let results=join_all(vector_tasks).await;
        for res in results{
            res?;
        }
       Ok(true)
    }
    pub async fn setScreenSize(azs_db:Arc<Mutex<AzsDb>>,screen_size: ScreenSize)->Result<bool, MyError>{
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let query=format!("UPDATE loc_const SET value=\"{},{}\" WHERE descr_var=\"cnst_ScreenSize\";",screen_size.width,screen_size.height);
        let res= sqlx::query(query.as_str())
            .execute(&mysqlpool)
            .await;
        match res {
            Ok(_) => {
                Ok(true)
            }
            Err(e) => {
                let str_error = format!("MYSQL|| {} error: SET SCREEEN SIZE \n", get_nowtime_str());
                Err(MyError::DatabaseError(str_error))
            }
        }
    }
    pub async fn checkAuth(azs_db:Arc<Mutex<AzsDb>>,id_user:i32,password:String,is_admin:&mut bool)->Result<bool, MyError>{
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let passwords:Vec<String>= sqlx::query_scalar::<_,String>("SELECT password FROM loc_users WHERE id_user=?;")
            .bind(id_user)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
        if !passwords.is_empty() && passwords[0]==password{
            if id_user>= 1000000 {
                *is_admin = true;
            }
            Ok(true)
        }else{
            Ok(false)
        }

        // let users= sqlx::query("SELECT * FROM loc_users INNER JOIN ref_users ON loc_users.id_user = ref_users.id_user;")
        //     .execute(self.mysql.as_ref().unwrap())
        //     .await;
        // match users {
        //     Ok(_) => {
        //         Ok(true)
        //     }
        //     Err(e) => {
        //         let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
        //         Err(MyError::DatabaseError(str_error))
        //     }
        // }

    }
    pub async fn getScreenSize(azs_db:Arc<Mutex<AzsDb>>)->Result<ScreenSize, MyError>{
        let azs_db=azs_db.lock().await;
        let mysqlpool=azs_db.mysql.as_ref().unwrap().clone();
        drop(azs_db);
        let screen_size:Vec<String>= sqlx::query_scalar::<_,String>("SELECT value FROM loc_const WHERE descr_var=\"cnst_ScreenSize\";;")
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::DatabaseError(str_error)
            })?;
        let mut screen_empty=ScreenSize{width:0,height:0};
        if !screen_size.is_empty(){
            let parts: Vec<&str> = screen_size[0].split(',').collect();
            if parts.len()!=2{
                let str_error = format!("MYSQL|| {} error: PARSE SCREEN  \n", get_nowtime_str());
                return Err(MyError::DatabaseError(str_error));
            }
            screen_empty.width=parts[0].parse::<i32>().map_err(|e| {
                let str_error = format!("MYSQL|| {} error: PARSE SCREEN WIDTH ERROR \n", get_nowtime_str());
                MyError::DatabaseError(str_error)
            }).unwrap_or(0);
            screen_empty.height=parts[0].parse::<i32>().map_err(|e| {
                let str_error = format!("MYSQL|| {} error: PARSE SCREEN HEIGHT ERROR \n", get_nowtime_str());
                MyError::DatabaseError(str_error)
            }).unwrap_or(0);
            Ok(screen_empty)
        }else{
            Ok(screen_empty)
        }
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
