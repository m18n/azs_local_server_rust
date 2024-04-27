
use crate::models::{MysqlInfo, User};
use ramhorns::{Template, Content};
#[derive(Content)]
pub struct AuthTemplate {
    pub smena:bool,
    pub users: Vec<User>,
}
#[derive(Content)]
pub struct ErrorDb {
    pub error:String
}
#[derive(Content)]
pub struct MysqlInfowithErrorDb {
    pub mysql_info_last:MysqlInfo,
    pub mysql_info_success:MysqlInfo,
    pub error_db:String
}