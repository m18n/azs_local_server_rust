use serde::{Deserialize, Serialize};

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