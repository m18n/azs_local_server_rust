use actix_web::{get, HttpResponse, Responder, web};
use ramhorns::Template;
use crate::base::file_openString;
use crate::models::{MyError};
use crate::render_temps::{AuthTemplate, ErrorDb, MysqlInfowithErrorDb};
use crate::StateDb;
//BASE URL /view/old
#[get("/login")]
pub async fn login(state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    let mut azs_db=state.azs_db.lock().await;
    let users = azs_db.getUsers().await?;
    let auth = AuthTemplate {
        smena: true,
        users: users
    };
    let contents = file_openString("./azs_site/public/public/old/login.html").await;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&auth)))
}
//BASE URL /view/old/userspace
#[get("/main")]
pub async fn main(state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    let mut azs_db=state.azs_db.lock().await;
    // let users = azs_db.getUsers().await?;
    // let auth = AuthTemplate {
    //     smena: true,
    //     users: users
    // };
    let contents = file_openString("./azs_site/public/public/old/serv.html").await;
   // let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
