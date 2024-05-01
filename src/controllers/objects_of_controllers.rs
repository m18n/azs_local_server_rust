use serde::{Deserialize, Serialize};
use crate::models::{ScreenSize, Tank, Tovar, Trk};

#[derive(Deserialize,Serialize)]
pub struct RequestResult{
    pub status:bool,
}
#[derive(Deserialize,Serialize)]
pub struct AuthInfo{
    pub id_user:i32,
    pub password:String
}
#[derive(Deserialize,Serialize)]
pub struct AuthResult{
    pub id_user:i32,
    pub password:String
}
#[derive(Deserialize,Serialize)]
pub struct AllObject{
    #[serde(default)]
    pub trks:Option<Vec<Trk>>,
    #[serde(default)]
    pub tovars:Option<Vec<Tovar>>,
    #[serde(default)]
    pub tanks:Option<Vec<Tank>>
}