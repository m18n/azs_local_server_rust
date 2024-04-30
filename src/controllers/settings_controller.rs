use actix_web::{get, HttpResponse, Responder, web};
use ramhorns::Template;
use crate::{render_temps, StateDb};
use crate::base::file_openString;
use crate::globals::LOGS_DB_ERROR;

#[get("/dberror")]
pub async fn m_show_error(state: web::Data<StateDb>)->impl Responder
{
    let azs_db=state.azs_db.lock().await;
    let error=render_temps::ErrorDb{error:LOGS_DB_ERROR.lock().await.clone()};
    let contents= file_openString("./azs_site/public/public/old/error_db.html").await;
    let tpl = Template::new(contents).unwrap();
    HttpResponse::Ok().content_type("text/html").body(tpl.render(&error))
}
#[get("/dbproperties")]
pub async fn m_show_properties(state: web::Data<StateDb>)->impl Responder
{
    let azs_db=state.azs_db.lock().await;
    let ctx=render_temps::MysqlInfowithErrorDb{mysql_info_last:azs_db.mysql_info_last.clone(),mysql_info_success:azs_db.mysql_info_success.clone(),error_db:LOGS_DB_ERROR.lock().await.clone()};
    let contents= file_openString("./azs_site/public/public/old/settings_db_error.html").await;
    let tpl = Template::new(contents).unwrap();
    HttpResponse::Ok().content_type("text/html").body(tpl.render(&ctx))
}
