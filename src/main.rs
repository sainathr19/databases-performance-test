use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Database Performance Test Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(home)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
