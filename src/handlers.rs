use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::models::*;

#[get("/data")]
pub async fn index(
    app_state: web::Data<AppState>,
) -> impl Responder {
    use crate::schema::lab1::dsl::*;

    dotenv::dotenv().ok();

    let web_block_result = web::block(move || {
        let mut conn = app_state.db_pool.get().unwrap();
        
        lab1.load::<Lab1Data>(&mut conn)
    })
    .await;

    if let Err(err) = web_block_result {
        eprintln!("{}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let query_result = web_block_result.unwrap();

    if let Err(err) = query_result {
        eprintln!("{}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let queried_lab1_data = query_result.unwrap();

    println!("Queried lab1 data: {:?}", queried_lab1_data);

    HttpResponse::Ok().json(queried_lab1_data)
}