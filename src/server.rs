mod recipe;
use recipe::sourdough;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn recipe(req: HttpRequest) -> impl Responder {
    let flower = req.match_info().get("name").unwrap_or("1000").parse::<i32>().expect("invalid value");
    sourdough(flower,70,20,2,5)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{flower}", web::get().to(recipe))
            .route("/", web::get().to(recipe))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

