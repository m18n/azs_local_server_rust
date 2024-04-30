use std::collections::HashMap;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use ramhorns::Template;
use crate::base::file_openString;
use crate::controllers::core_logic_controller::{start_controller, wrap_handler};
use crate::models::{get_nowtime_str, MyError, Pist, ScreenSize, Tank, Tovar, Trk};
use crate::render_temps::{AuthTemplate, ErrorDb, MainTemplate, MysqlInfowithErrorDb, PistForTemplate, TrkForTemplate};
use crate::{main, StateDb};
//BASE URL /view/old
#[get("/login")]
pub async fn m_login(state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
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
//BASE URL /view/userspace/old
async fn get_trks_for_template(tovars: &Vec<Tovar>, tanks: &Vec<Tank>, trks: &Vec<Trk>,screen_size: ScreenSize) -> Result<Vec<TrkForTemplate>, MyError> {
    // Створення HashMap для швидкого доступу до Tovar по id_tovar
    let tovar_map: HashMap<i32, &Tovar> = tovars.iter().map(|t| (t.id_tovar, t)).collect();

    // Створення HashMap для швидкого доступу до Tank по id_tank
    let tank_map: HashMap<i32, &Tank> = tanks.iter().map(|t| (t.id_tank, t)).collect();

    let mut trks_template: Vec<TrkForTemplate> = Vec::with_capacity(trks.len());

    for trk in trks {
        let mut pists_: Vec<PistForTemplate> = Vec::with_capacity(trk.pists.len());

        for pist in &trk.pists {
            if let Some(tank) = tank_map.get(&pist.id_tank) {
                if let Some(tovar) = tovar_map.get(&tank.id_tovar) {
                    let pist_template = PistForTemplate {
                        id_pist: pist.id_pist,
                        id_tank: pist.id_tank,
                        price: tovar.price,
                        r: tovar.color.r,
                        g: tovar.color.g,
                        b: tovar.color.b,
                        name: tovar.name.clone()
                    };
                    pists_.push(pist_template);
                } else {
                    let str_error = format!("MYSQL|| {} error: PARSE TOVAR\n", get_nowtime_str());
                    return Err(MyError::DatabaseError(str_error));

                }
            } else {
                let str_error = format!("MYSQL|| {} error: PARSE TANKS\n", get_nowtime_str());
                return Err(MyError::DatabaseError(str_error));
            }
        }
        let x_pos_f=trk.x_pos as f32;
        let y_pos_f=trk.y_pos as f32;
        let screen_width_f=screen_size.width as f32;
        let trk_template = TrkForTemplate {
            nn: trk.nn,
            id_trk: trk.id_trk,
            x_pos: x_pos_f/screen_width_f*100.0,
            y_pos: y_pos_f/screen_width_f*100.0,
            scale: trk.scale,
            pists: pists_
        };
        trks_template.push(trk_template);
    }
    Ok(trks_template)
}
pub async fn a_main(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    let mut azs_db=state.azs_db.lock().await;
    let screen=azs_db.getScreenSize().await?;
    let tovars=azs_db.getTovars().await?;
    let tanks=azs_db.getTanks().await?;
    let trks=azs_db.getTrks().await?;
    let trks_for_template=get_trks_for_template(&tovars,&tanks,&trks,screen.clone()).await?;
    let main_template=MainTemplate{
        admin:true,
        screen_width:screen.width,
        trks:trks_for_template
    };
    let tovars=azs_db.getTovars().await?;
    println!("TOVARS: {:?}",&tovars);
    let contents = file_openString("./azs_site/public/public/old/serv.html").await;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&main_template)))
}
pub async fn u_main(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    let mut azs_db=state.azs_db.lock().await;
    let screen=azs_db.getScreenSize().await?;
    let tovars=azs_db.getTovars().await?;
    let tanks=azs_db.getTanks().await?;
    let trks=azs_db.getTrks().await?;
   
    let trks_for_template=get_trks_for_template(&tovars,&tanks,&trks,screen.clone()).await?;
    let main_template=MainTemplate{
        admin:false,
        screen_width:screen.width,
        trks:trks_for_template
    };
    let contents = file_openString("./azs_site/public/public/old/serv.html").await;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&main_template)))
}

#[get("/main")]
pub async fn m_main(req:HttpRequest,state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    //let mut azs_db=state.azs_db.lock().await;
    // let users = azs_db.getUsers().await?;
    // let auth = AuthTemplate {
    //     smena: true,
    //     users: users
    // };
    // let screen=azs_db.getScreenSize().await?;
    // let tanks=azs_db.getTanks().await?;
    // println!("SCREEN: {:?}",&screen);
    // println!("TANKS: {:?}",&tanks);
    start_controller(wrap_handler(a_main), wrap_handler(u_main),&req,state.clone()).await
    //let trks=azs_db.getTrks().await?;
    // println!("TANKS: {:?}",&trks);
    // let contents = file_openString("./azs_site/public/public/old/serv.html").await;
    // //let tpl = Template::new(contents).unwrap();
    // Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}