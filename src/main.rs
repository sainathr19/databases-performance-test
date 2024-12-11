mod db;
mod helpers;
mod models;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use helpers::cron::fetch_latest_data;


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Database Performance Test Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Start Cron JOb to Fetch Latest Data
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
        loop {
            interval.tick().await;
            println!("Fetching Latest Data...");
            let intervals = fetch_latest_data().await;
            match  intervals {
                Ok(_)=>{
                    println!("Success");
                },Err(err)=>{
                    println!("ERROR : {:?}",err)
                }
                
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .service(home)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
